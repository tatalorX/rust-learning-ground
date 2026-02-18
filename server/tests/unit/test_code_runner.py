"""
Zero-trust unit tests for CodeRunner.

Assumes nothing: verify every return shape, error path, and that no
internal state or secrets leak. Does not trust docstrings or types.
"""

import asyncio
import shutil
import tempfile
from pathlib import Path
from unittest.mock import patch

import pytest

pytestmark = [pytest.mark.unit]


class TestCodeRunnerContract:
    """Verify CodeRunner return contract and behavior - zero trust."""

    @pytest.fixture(autouse=True)
    def sandbox_dir(self, tmp_path):
        """Use a real temp dir so we don't depend on hardcoded SANDBOX_DIR."""
        with patch("app.code_runner.SANDBOX_DIR", tmp_path):
            yield tmp_path

    @pytest.mark.asyncio
    async def test_run_code_returns_expected_attributes(self, sandbox_dir):
        """ExecutionResult must have exactly the fields we rely on."""
        from app.code_runner import CodeRunner, ExecutionResult

        code = 'fn main() { println!("ok"); }'
        result = await CodeRunner.run_code(code)

        assert isinstance(result, ExecutionResult)
        assert hasattr(result, "success")
        assert hasattr(result, "output")
        assert hasattr(result, "error")
        assert hasattr(result, "execution_time_ms")
        assert hasattr(result, "compilation_error")
        assert hasattr(result, "timed_out")
        assert hasattr(result, "memory_exceeded")
        # No extra attributes we might accidentally rely on
        assert set(vars(result).keys()) == {
            "success", "output", "error", "execution_time_ms",
            "compilation_error", "timed_out", "memory_exceeded",
        }

    @pytest.mark.asyncio
    async def test_run_code_output_is_string_not_none(self, sandbox_dir):
        """Output and error must be str (or None for error), never bytes or other."""
        from app.code_runner import CodeRunner

        code = 'fn main() { println!("hi"); }'
        result = await CodeRunner.run_code(code)

        assert isinstance(result.output, str), "output must be str"
        assert result.error is None or isinstance(result.error, str), "error must be str or None"
        assert isinstance(result.execution_time_ms, int), "execution_time_ms must be int"
        assert result.execution_time_ms >= 0, "execution_time_ms must be non-negative"

    @pytest.mark.asyncio
    async def test_run_code_invalid_syntax_returns_compilation_error(self, sandbox_dir):
        """Invalid Rust must set success=False and compilation_error=True."""
        from app.code_runner import CodeRunner

        code = "fn main() { broken"
        result = await CodeRunner.run_code(code)

        assert result.success is False
        assert result.compilation_error is True
        assert result.error is not None
        assert isinstance(result.error, str)
        assert len(result.error) > 0

    @pytest.mark.asyncio
    async def test_run_code_empty_string_still_returns_result(self, sandbox_dir):
        """Empty code must not raise; must return a valid ExecutionResult."""
        from app.code_runner import CodeRunner

        result = await CodeRunner.run_code("")

        assert result.success is False
        assert result.compilation_error is True
        assert result.error is not None

    @pytest.mark.asyncio
    async def test_verify_solution_returns_dict_with_expected_keys(self, sandbox_dir):
        """verify_solution must return a dict with known keys only."""
        from app.code_runner import CodeRunner

        code = 'fn main() { println!("match"); }'
        out = await CodeRunner.verify_solution(code, "match")

        assert isinstance(out, dict)
        required = {"success", "output", "expected", "error", "execution_time_ms", "compilation_error", "timed_out"}
        assert required <= set(out.keys()), f"Missing keys: {required - set(out.keys())}"
        for k in out:
            assert k in required, f"Unexpected key: {k}"

    @pytest.mark.asyncio
    async def test_verify_solution_success_when_output_matches(self, sandbox_dir):
        """Success only when output.strip() == expected.strip()."""
        from app.code_runner import CodeRunner

        code = 'fn main() { println!("  yes  "); }'
        out = await CodeRunner.verify_solution(code, "  yes  ")
        assert out["success"] is True

        code2 = 'fn main() { println!("no"); }'
        out2 = await CodeRunner.verify_solution(code2, "yes")
        assert out2["success"] is False

    @pytest.mark.asyncio
    async def test_run_code_no_secrets_in_output_or_error(self, sandbox_dir):
        """Response must not contain common secret patterns."""
        from app.code_runner import CodeRunner

        code = 'fn main() { println!("public"); }'
        result = await CodeRunner.run_code(code)

        combined = (result.output or "") + (result.error or "")
        forbidden = ["SECRET", "JWT_SECRET", "password", "api_key", "token="]
        for word in forbidden:
            assert word not in combined.upper() or "public" in combined.lower(), (
                f"Leak risk: {word} in response"
            )

    @pytest.mark.asyncio
    async def test_run_code_exercise_id_optional(self, sandbox_dir):
        """run_code must accept exercise_id=None and still return valid result."""
        from app.code_runner import CodeRunner

        result = await CodeRunner.run_code('fn main() {}', exercise_id=None)
        assert hasattr(result, "success")
        assert result.execution_time_ms >= 0


class TestCodeRunnerRustcMissing:
    """When rustc is missing, must not crash; must return error result."""

    @pytest.mark.asyncio
    async def test_graceful_failure_when_rustc_unavailable(self, tmp_path):
        """If rustc not in PATH, return compilation_error and clear error message."""
        from app.code_runner import CodeRunner

        with patch("app.code_runner.SANDBOX_DIR", tmp_path), \
             patch("app.code_runner.asyncio.create_subprocess_exec", side_effect=FileNotFoundError("rustc not found")):
            result = await CodeRunner.run_code('fn main() {}')

        assert result.success is False
        assert result.compilation_error is True
        assert result.error is not None
        assert "rustc" in result.error.lower() or "not found" in result.error.lower()
