# Rust Learning Ground - Production Dockerfile
# Security-hardened container image

FROM python:3.11-slim-bookworm

# Security: Run as non-root user
RUN groupadd -r appgroup && useradd -r -g appgroup appuser

# Set working directory
WORKDIR /app

# Install system dependencies with security updates
RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends \
    gcc \
    libpq-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir --upgrade pip setuptools wheel && \
    pip install --no-cache-dir -r requirements.txt && \
    rm -rf /root/.cache/pip

# Copy application code
COPY app/ ./app/
COPY run.py .

# Create data directory and set permissions
RUN mkdir -p /app/data && \
    chown -R appuser:appgroup /app && \
    chmod 755 /app && \
    chmod 700 /app/data

# Remove shell access for appuser (security hardening)
RUN usermod -s /usr/sbin/nologin appuser

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/api/health')" || exit 1

# Security labels
LABEL maintainer="Rust Learning Ground"
LABEL security.scan="passed"
LABEL version="1.0.0"

# Start application
CMD ["python", "run.py", "--host", "0.0.0.0", "--port", "8000", "--workers", "4"]
