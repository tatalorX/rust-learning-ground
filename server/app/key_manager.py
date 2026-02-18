# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Centralized encryption key management for the Rust Learning Ground platform.

This module handles automatic generation, storage, and rotation of:
- Master encryption key (AES-256-GCM)
- JWT signing secrets
- Cookie/Session secrets
- Database encryption keys

All keys are derived from a single master key for easy backup/restore.
"""

import os
import secrets
import hashlib
import base64
from pathlib import Path
from typing import Optional, Dict, Tuple
from cryptography.fernet import Fernet
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.backends import default_backend


class KeyManager:
    """
    Manages all cryptographic keys for the application.
    
    Uses a single master key to derive all other keys using HKDF-like derivation.
    This allows for:
    - Easy backup (just backup the master key)
    - Automatic key rotation
    - Consistent key generation across restarts
    """
    
    def __init__(self, keys_dir: Optional[str] = None):
        """Initialize the key manager."""
        self.keys_dir = Path(keys_dir) if keys_dir else Path(__file__).parent.parent / "data" / "keys"
        self.keys_dir.mkdir(parents=True, exist_ok=True)
        
        self._master_key: Optional[bytes] = None
        self._derived_keys: Dict[str, str] = {}
        
    def _get_master_key_file(self) -> Path:
        """Get the path to the master key file."""
        return self.keys_dir / "master.key"
    
    def _generate_master_key(self) -> bytes:
        """Generate a new 256-bit master key."""
        return secrets.token_bytes(32)
    
    def _load_or_create_master_key(self) -> bytes:
        """Load existing master key or create a new one."""
        key_file = self._get_master_key_file()
        
        if key_file.exists():
            # Load existing key
            key_data = key_file.read_bytes()
            # Key is base64 encoded for storage
            return base64.urlsafe_b64decode(key_data)
        
        # Generate new master key
        master_key = self._generate_master_key()
        
        # Save with secure permissions (readable only by owner)
        key_file.write_bytes(base64.urlsafe_b64encode(master_key))
        os.chmod(key_file, 0o600)
        
        # Also create a backup notice
        readme_file = self.keys_dir / "README.txt"
        readme_file.write_text(
            "IMPORTANT: Backup your master.key file!\n"
            "\n"
            "This key is used to derive all other encryption keys.\n"
            "If you lose this key, all encrypted data will be unrecoverable.\n"
            "\n"
            "To backup: Copy master.key to a secure location.\n"
            "To restore: Place the backed-up master.key in this directory.\n"
            "\n"
            f"Key directory: {self.keys_dir.absolute()}\n"
        )
        
        return master_key
    
    def _derive_key(self, purpose: str, length: int = 32) -> bytes:
        """
        Derive a key from the master key for a specific purpose.
        
        Uses HKDF-like derivation with SHA-256.
        """
        if self._master_key is None:
            self._master_key = self._load_or_create_master_key()
        
        # Use PBKDF2HMAC for key derivation
        kdf = PBKDF2HMAC(
            algorithm=hashes.SHA256(),
            length=length,
            salt=purpose.encode(),  # Purpose acts as salt
            iterations=100000,
            backend=default_backend()
        )
        
        return kdf.derive(self._master_key)
    
    def get_jwt_secret(self) -> str:
        """Get or generate the JWT signing secret."""
        if "jwt" not in self._derived_keys:
            key = self._derive_key("jwt-signing-v1", 64)
            self._derived_keys["jwt"] = base64.urlsafe_b64encode(key).decode()
        return self._derived_keys["jwt"]
    
    def get_session_secret(self) -> str:
        """Get or generate the session/cookie secret."""
        if "session" not in self._derived_keys:
            key = self._derive_key("session-cookie-v1", 64)
            self._derived_keys["session"] = base64.urlsafe_b64encode(key).decode()
        return self._derived_keys["session"]
    
    def get_csrf_secret(self) -> str:
        """Get or generate the CSRF token secret."""
        if "csrf" not in self._derived_keys:
            key = self._derive_key("csrf-token-v1", 64)
            self._derived_keys["csrf"] = base64.urlsafe_b64encode(key).decode()
        return self._derived_keys["csrf"]
    
    def get_encryption_key(self) -> bytes:
        """Get or generate the Fernet encryption key for database fields."""
        if "encryption" not in self._derived_keys:
            key = self._derive_key("field-encryption-v1", 32)
            # Fernet needs 32 bytes base64 encoded (which becomes 44 chars)
            self._derived_keys["encryption"] = base64.urlsafe_b64encode(key)
        return self._derived_keys["encryption"]
    
    def get_fernet(self) -> Fernet:
        """Get a Fernet instance for encryption/decryption."""
        return Fernet(self.get_encryption_key())
    
    def encrypt_field(self, plaintext: str) -> str:
        """Encrypt a field value using Fernet."""
        f = self.get_fernet()
        return f.encrypt(plaintext.encode()).decode()
    
    def decrypt_field(self, ciphertext: str) -> str:
        """Decrypt a field value using Fernet."""
        f = self.get_fernet()
        return f.decrypt(ciphertext.encode()).decode()
    
    def rotate_keys(self) -> None:
        """
        Generate new master key (for emergency key rotation).
        
        WARNING: This will invalidate all existing sessions and encrypted data!
        Only use in emergency situations.
        """
        # Backup old key
        key_file = self._get_master_key_file()
        if key_file.exists():
            backup_file = self.keys_dir / f"master.key.backup.{int(os.time())}"
            key_file.rename(backup_file)
        
        # Clear derived keys cache
        self._derived_keys = {}
        self._master_key = None
        
        # Generate new key
        self._master_key = self._generate_master_key()
        key_file.write_bytes(base64.urlsafe_b64encode(self._master_key))
        os.chmod(key_file, 0o600)
    
    def export_key(self) -> str:
        """Export the master key for backup (base64 encoded)."""
        if self._master_key is None:
            self._master_key = self._load_or_create_master_key()
        return base64.urlsafe_b64encode(self._master_key).decode()
    
    def import_key(self, key_b64: str) -> None:
        """Import a master key from backup."""
        key_file = self._get_master_key_file()
        key_file.write_bytes(key_b64.encode())
        os.chmod(key_file, 0o600)
        self._master_key = base64.urlsafe_b64decode(key_b64)
        self._derived_keys = {}


# Global key manager instance
_key_manager: Optional[KeyManager] = None


def get_key_manager() -> KeyManager:
    """Get the global key manager instance."""
    global _key_manager
    if _key_manager is None:
        _key_manager = KeyManager()
    return _key_manager


def get_jwt_secret() -> str:
    """Get the JWT signing secret."""
    return get_key_manager().get_jwt_secret()


def get_session_secret() -> str:
    """Get the session/cookie secret."""
    return get_key_manager().get_session_secret()


def get_csrf_secret() -> str:
    """Get the CSRF token secret."""
    return get_key_manager().get_csrf_secret()


def get_encryption_key() -> bytes:
    """Get the Fernet encryption key."""
    return get_key_manager().get_encryption_key()


def encrypt_field(plaintext: str) -> str:
    """Encrypt a database field."""
    return get_key_manager().encrypt_field(plaintext)


def decrypt_field(ciphertext: str) -> str:
    """Decrypt a database field."""
    return get_key_manager().decrypt_field(ciphertext)


# For CLI usage
if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Key management for Rust Learning Ground")
    parser.add_argument("--export", action="store_true", help="Export master key for backup")
    parser.add_argument("--import-key", metavar="KEY", help="Import master key from backup")
    parser.add_argument("--rotate", action="store_true", help="Rotate keys (WARNING: invalidates sessions!)")
    parser.add_argument("--show-derived", action="store_true", help="Show derived key hashes (for verification)")
    
    args = parser.parse_args()
    
    km = get_key_manager()
    
    if args.export:
        print("Master Key (save this securely!):")
        print(km.export_key())
        print("\nWARNING: Keep this key safe! Anyone with this key can decrypt your data.")
    
    elif args.import_key:
        km.import_key(args.import_key)
        print("Master key imported successfully.")
    
    elif args.rotate:
        confirm = input("WARNING: This will invalidate all sessions and encrypted data!\nType 'ROTATE' to confirm: ")
        if confirm == "ROTATE":
            km.rotate_keys()
            print("Keys rotated successfully.")
        else:
            print("Cancelled.")
    
    elif args.show_derived:
        print("Derived Key Hashes (for verification):")
        print(f"  JWT:     {hashlib.sha256(km.get_jwt_secret().encode()).hexdigest()[:16]}...")
        print(f"  Session: {hashlib.sha256(km.get_session_secret().encode()).hexdigest()[:16]}...")
        print(f"  CSRF:    {hashlib.sha256(km.get_csrf_secret().encode()).hexdigest()[:16]}...")
    
    else:
        print("Key manager ready.")
        print(f"Keys directory: {km.keys_dir}")
        print(f"Master key exists: {km._get_master_key_file().exists()}")
