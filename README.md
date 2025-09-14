# search-service
search-service# Search Service  Centralized search for the **Gitdigital Products** ecosystem.   Indexes content from profiles, files, logs, and other services.  ## 🚀 Features - `POST /index` → Add an item to the search index - `GET /search/:query` → Search indexed items - Tracks content + source type 
# Search Service

Centralized search for the **Gitdigital Products** ecosystem.  
Indexes content from profiles, files, logs, and other services.

## 🚀 Features
- `POST /index` → Add an item to the search index
- `GET /search/:query` → Search indexed items
- Tracks content + source type
- In-memory index with simple substring matching

## 🛠️ Setup
```bash
cargo run
