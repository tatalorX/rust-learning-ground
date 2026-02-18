# Makefile for Rust Learning Ground Authentication Server

.PHONY: help setup dev prod test clean logs

help: ## Show this help message
	@echo "Rust Learning Ground - Auth Server"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Initial setup (create venv, install deps)
	./setup.sh

dev: ## Run development server with auto-reload
	. venv/bin/activate && python run.py --reload

prod: ## Run production server with 4 workers
	. venv/bin/activate && python run.py --workers 4

docker-build: ## Build Docker image
	docker-compose build

docker-up: ## Start with Docker Compose
	docker-compose up -d

docker-down: ## Stop Docker containers
	docker-compose down

docker-logs: ## View Docker logs
	docker-compose logs -f

test: ## Run tests (if any)
	. venv/bin/activate && python -m pytest

lint: ## Run linting
	. venv/bin/activate && flake8 app/
	. venv/bin/activate && black --check app/

format: ## Format code
	. venv/bin/activate && black app/

clean: ## Clean up cache and temp files
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete
	find . -type f -name "*.pyo" -delete
	find . -type f -name ".coverage" -delete
	docker system prune -f 2>/dev/null || true

backup: ## Backup database
	mkdir -p backups
	cp data/app.db backups/app.db.$$(date +%Y%m%d_%H%M%S)
	@echo "✅ Database backed up to backups/"

shell: ## Open Python shell with app context
	. venv/bin/activate && python -c "from app.database import init_db; import asyncio; asyncio.run(init_db()); print('Database initialized')"

health: ## Check server health
	curl -s http://localhost:8000/api/health | python -m json.tool
