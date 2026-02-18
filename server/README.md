# 🔐 Rust Learning Ground - Authentication Server

A **secure, production-ready authentication system** for the Rust Learning Ground platform.

## ✨ Features

### Security (Industry Best Practices)

- **🔐 Argon2 Password Hashing** - OWASP-recommended memory-hard password hashing
- **🎟️ JWT Tokens** - Short-lived access tokens with refresh token rotation
- **🍪 HttpOnly Cookies** - Prevents XSS token theft
- **🛡️ CSRF Protection** - State parameter validation for OAuth
- **⚡ Rate Limiting** - Prevents brute force attacks
- **📊 Audit Logging** - Tracks all security-relevant events
- **🔒 Security Headers** - CSP, HSTS, X-Frame-Options, etc.
- **🚫 Account Lockout** - Auto-lock after failed login attempts

### Authentication Methods

- **Email/Password** - Traditional registration with strong password requirements
- **GitHub OAuth** - One-click login with GitHub
- **Token Refresh** - Seamless session management
- **Multi-device Support** - Login from multiple devices, revoke individually

### User Features

- **Individual Progress Tracking** - Each user has isolated progress data
- **XP & Streak System** - Gamified learning experience
- **Cloud Code Sync** - Optional code backup per problem
- **Profile Management** - Update display name, view stats

## 🚀 Quick Start

### Development

```bash
cd server
python3 -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt
python run.py --reload
```

The server will be available at `http://localhost:8000`

### Production (Docker)

```bash
cd server
cp .env.example .env
# Edit .env with your secrets
docker-compose up -d
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed production setup.

## 📁 Project Structure

```
server/
├── app/
│   ├── __init__.py
│   ├── main.py           # FastAPI application & routes
│   ├── config.py         # Configuration & settings
│   ├── database.py       # Database connection & models
│   ├── models.py         # SQLAlchemy ORM models
│   ├── schemas.py        # Pydantic request/response schemas
│   ├── security.py       # Password hashing, JWT, CSRF
│   ├── auth_service.py   # Business logic for auth
│   └── github_oauth.py   # GitHub OAuth integration
├── data/                 # Database files (SQLite)
├── .env.example          # Example environment variables
├── docker-compose.yml    # Docker deployment config
├── Dockerfile            # Docker image definition
├── requirements.txt      # Python dependencies
├── run.py               # Startup script
└── DEPLOYMENT.md        # Production deployment guide
```

## 🔑 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SECRET_KEY` | App secret for CSRF/session | Auto-generated |
| `JWT_SECRET_KEY` | JWT signing key | Auto-generated |
| `DATABASE_URL` | Database connection string | SQLite |
| `GITHUB_CLIENT_ID` | GitHub OAuth app ID | None |
| `GITHUB_CLIENT_SECRET` | GitHub OAuth app secret | None |
| `COOKIE_SECURE` | HTTPS-only cookies | `false` |
| `CORS_ORIGINS` | Allowed frontend origins | Localhost |

## 🔒 Security Architecture

### Password Security

- **Argon2id** hashing (resistant to GPU/ASIC attacks)
- Minimum 8 characters, requires uppercase, lowercase, number
- Password history not stored (hash-only)
- Account lockout after 5 failed attempts (30 min)

### Token Security

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Client    │────▶│   Access    │────▶│   Refresh   │
│  (Browser)  │◄────│   Token     │◄────│   Token     │
└─────────────┘     └─────────────┘     └─────────────┘
                           │                   │
                           ▼                   ▼
                    Expires: 15 min      Expires: 7 days
                    HttpOnly cookie      HttpOnly cookie
                    Stored in memory     Stored in database
```

### Session Flow

1. **Login** → Server sets `access_token` (15min) and `refresh_token` (7days) cookies
2. **API Requests** → Client sends cookies automatically, server validates JWT
3. **Token Expiry** → Client uses refresh endpoint with `refresh_token` to get new tokens
4. **Logout** → Server revokes refresh token, clears cookies

## 📡 API Endpoints

### Authentication

| Endpoint | Method | Description | Rate Limit |
|----------|--------|-------------|------------|
| `/api/auth/register` | POST | Create new account | 3/minute |
| `/api/auth/login` | POST | Authenticate | 5/minute |
| `/api/auth/refresh` | POST | Refresh access token | - |
| `/api/auth/logout` | POST | Logout current device | - |
| `/api/auth/logout-all` | POST | Logout all devices | - |
| `/api/auth/github` | GET | Get GitHub OAuth URL | - |
| `/api/auth/github/callback` | GET | GitHub OAuth callback | - |

### User

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/me` | GET | Get current user profile |
| `/api/stats` | GET | Get user stats (public) |
| `/api/progress` | GET | Get detailed progress |
| `/api/progress/solve` | POST | Submit solved problem |
| `/api/csrf` | GET | Get CSRF token |
| `/api/health` | GET | Health check |

## 🛡️ Security Headers

The server sends these security headers on all responses:

```
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'; ...
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: geolocation=(), microphone=(), camera=()
```

## 🧪 Testing

```bash
# Run the development server
python run.py --reload

# Test health endpoint
curl http://localhost:8000/api/health

# Test registration
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@example.com","password":"Test1234"}'
```

## 🚢 Deployment Checklist

- [ ] Generate strong `SECRET_KEY` and `JWT_SECRET_KEY`
- [ ] Set `ENVIRONMENT=production`
- [ ] Enable HTTPS (`COOKIE_SECURE=true`)
- [ ] Configure `CORS_ORIGINS` to your domain only
- [ ] Set up PostgreSQL for production (optional but recommended)
- [ ] Configure GitHub OAuth (optional)
- [ ] Set up automated backups
- [ ] Configure log monitoring
- [ ] Test account lockout functionality
- [ ] Verify security headers in browser

## 📄 License

MIT - See main project LICENSE

## 🤝 Contributing

This is part of the Rust Learning Ground project. See the main repository for contribution guidelines.
