import { describe, it, expect, beforeEach } from "vitest";
import "../../../src/components/organisms/CodeEditor";

describe("RLCodeEditor", () => {
  beforeEach(() => {
    document.body.innerHTML = "";
  });

  it("renders with default attributes", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor");
    expect(editor).toBeTruthy();
  });

  it("renders with initial value", () => {
    const initialValue = 'fn main() { println!("Hello"); }';
    document.body.innerHTML = `<rl-code-editor value="${initialValue}"></rl-code-editor>`;
    const editor = document.querySelector("rl-code-editor");
    expect(editor?.getAttribute("value")).toBe(initialValue);
  });

  it("renders line numbers", () => {
    document.body.innerHTML =
      '<rl-code-editor value="line 1\nline 2\nline 3"></rl-code-editor>';
    const editor = document.querySelector("rl-code-editor");
    const lineNumbers = editor?.shadowRoot?.querySelectorAll(".line-number");
    expect(lineNumbers?.length).toBe(3);
  });

  it("renders textarea element", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor");
    const textarea = editor?.shadowRoot?.querySelector("textarea");
    expect(textarea).toBeTruthy();
  });

  it("dispatches change event on input", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor");

    let changeEvent: CustomEvent | null = null;
    editor?.addEventListener("change", (e: Event) => {
      changeEvent = e as CustomEvent;
    });

    const textarea = editor?.shadowRoot?.querySelector("textarea");
    if (textarea) {
      textarea.value = "new value";
      textarea.dispatchEvent(new Event("input", { bubbles: true }));
    }

    expect(changeEvent).toBeTruthy();
    expect(changeEvent?.detail.value).toBe("new value");
  });

  it("supports readonly attribute", () => {
    document.body.innerHTML = "<rl-code-editor readonly></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor");
    const textarea = editor?.shadowRoot?.querySelector("textarea");
    expect(textarea?.hasAttribute("readonly")).toBe(true);
  });

  it("supports custom placeholder", () => {
    const placeholder = "Enter your code here";
    document.body.innerHTML = `<rl-code-editor placeholder="${placeholder}"></rl-code-editor>`;
    const editor = document.querySelector("rl-code-editor");
    const textarea = editor?.shadowRoot?.querySelector("textarea");
    expect(textarea?.getAttribute("placeholder")).toBe(placeholder);
  });

  it("dispatches run event on Ctrl+Enter", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor");

    let runEventFired = false;
    editor?.addEventListener("run", () => {
      runEventFired = true;
    });

    const textarea = editor?.shadowRoot?.querySelector("textarea");
    if (textarea) {
      const event = new KeyboardEvent("keydown", {
        key: "Enter",
        ctrlKey: true,
        bubbles: true,
      });
      textarea.dispatchEvent(event);
    }

    expect(runEventFired).toBe(true);
  });

  it("dispatches run event on Cmd+Enter", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor");

    let runEventFired = false;
    editor?.addEventListener("run", () => {
      runEventFired = true;
    });

    const textarea = editor?.shadowRoot?.querySelector("textarea");
    if (textarea) {
      const event = new KeyboardEvent("keydown", {
        key: "Enter",
        metaKey: true,
        bubbles: true,
      });
      textarea.dispatchEvent(event);
    }

    expect(runEventFired).toBe(true);
  });

  it("dispatches save event on Ctrl+S", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor");

    let saveEventFired = false;
    editor?.addEventListener("save", () => {
      saveEventFired = true;
    });

    const textarea = editor?.shadowRoot?.querySelector("textarea");
    if (textarea) {
      const event = new KeyboardEvent("keydown", {
        key: "s",
        ctrlKey: true,
        bubbles: true,
      });
      textarea.dispatchEvent(event);
    }

    expect(saveEventFired).toBe(true);
  });

  it("inserts tab on Tab key", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor") as any;

    const textarea = editor?.shadowRoot?.querySelector("textarea");
    if (textarea) {
      textarea.value = "";
      textarea.selectionStart = 0;
      textarea.selectionEnd = 0;

      const event = new KeyboardEvent("keydown", {
        key: "Tab",
        bubbles: true,
      });

      // Prevent default should be called
      const preventDefaultSpy = vi.fn();
      Object.defineProperty(event, "preventDefault", {
        value: preventDefaultSpy,
      });

      textarea.dispatchEvent(event);

      // Check if 4 spaces were inserted
      expect(textarea.value).toBe("    ");
    }
  });

  it("getValue returns current value", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor") as any;

    const textarea = editor?.shadowRoot?.querySelector("textarea");
    if (textarea) {
      textarea.value = "test code";
    }

    expect(editor.getValue()).toBe("test code");
  });

  it("setValue updates the textarea and line numbers", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor") as any;

    const newValue = "line 1\nline 2\nline 3\nline 4";
    editor.setValue(newValue);

    const textarea = editor?.shadowRoot?.querySelector("textarea");
    expect(textarea?.value).toBe(newValue);

    const lineNumbers = editor?.shadowRoot?.querySelectorAll(".line-number");
    expect(lineNumbers?.length).toBe(4);
  });

  it("focus method focuses the textarea", () => {
    document.body.innerHTML = "<rl-code-editor></rl-code-editor>";
    const editor = document.querySelector("rl-code-editor") as any;

    const focusSpy = vi.fn();
    const textarea = editor?.shadowRoot?.querySelector("textarea");
    if (textarea) {
      textarea.focus = focusSpy;
    }

    editor.focus();
    expect(focusSpy).toHaveBeenCalled();
  });

  it("updates line numbers when value changes", () => {
    document.body.innerHTML =
      '<rl-code-editor value="line 1"></rl-code-editor>';
    const editor = document.querySelector("rl-code-editor") as any;

    // Initially 1 line
    let lineNumbers = editor?.shadowRoot?.querySelectorAll(".line-number");
    expect(lineNumbers?.length).toBe(1);

    // Set value with 3 lines
    editor.setValue("line 1\nline 2\nline 3");

    lineNumbers = editor?.shadowRoot?.querySelectorAll(".line-number");
    expect(lineNumbers?.length).toBe(3);
  });
});
