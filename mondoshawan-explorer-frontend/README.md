# Mondoshawan Block Explorer - Frontend

A modern, responsive web interface for exploring the Mondoshawan blockchain.

## Features

- **Network Dashboard** - Real-time network statistics
- **Block Viewer** - Browse recent blocks with details
- **Transaction Viewer** - View recent transactions
- **Address Lookup** - Search and view address information
- **Search** - Search by block hash, transaction hash, or address
- **Auto-refresh** - Dashboard updates every 30 seconds

## Setup

1. **Start the Mondoshawan node:**
   ```bash
   cd mondoshawan-blockchain
   cargo run --bin node
   ```

2. **Open the frontend:**
   - Simply open `index.html` in a web browser
   - Or use a local web server:
     ```bash
     # Python 3
     python3 -m http.server 3000
     
     # Or Node.js
     npx http-server -p 3000
     ```

3. **Access the explorer:**
   - Open `http://localhost:3000` in your browser
   - The explorer will connect to the API at `http://localhost:8080`

## API Endpoints Used

- `GET /api/stats/network` - Network statistics
- `GET /api/stats/chain` - Chain statistics
- `GET /api/blocks/recent` - Recent blocks
- `GET /api/transactions/recent` - Recent transactions
- `GET /api/blocks/:identifier` - Block details
- `GET /api/transactions/:hash` - Transaction details
- `GET /api/addresses/:address` - Address details
- `GET /api/search?q=...` - Search

## Configuration

To change the API endpoint, edit `app.js`:

```javascript
const API_BASE = 'http://localhost:8080/api';
```

## Browser Support

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)

## Future Enhancements

- Real-time WebSocket updates
- Transaction history pagination
- Block detail pages
- Transaction detail pages
- Address transaction history
- Charts and graphs
- Export functionality

