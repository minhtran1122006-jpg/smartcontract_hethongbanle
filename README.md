# 🏪 Retail Management Blockchain System

[![Stellar Soroban](https://img.shields.io/badge/Stellar-Soroban-blue.svg)](https://soroban.stellar.org/)
[![React](https://img.shields.io/badge/Frontend-React_PWA-green.svg)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-✓-3178C6)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A comprehensive retail management system built on Stellar Soroban blockchain, featuring point-of-sale, inventory management, customer loyalty, and real-time analytics.

## 🌟 Features

### 🛒 POS System
- Real-time sales processing
- Multiple payment methods
- XMoney token integration
- Receipt generation
- Barcode scanning support

### 📦 Inventory Management  
- Real-time stock tracking
- Automated reordering alerts
- Supplier management
- Batch and expiry tracking
- Low stock notifications

### 👥 CRM & Loyalty
- Customer profile management
- Loyalty points system
- Marketing campaigns
- Customer behavior analytics
- Personalized offers

### 📊 Analytics Dashboard
- Real-time sales analytics
- Inventory insights and trends
- Customer behavior analysis
- Financial reporting
- Performance metrics

### 💰 Payment & Accounting
- Multi-currency support
- Automated accounting entries
- Tax calculation and reporting
- Payroll management
- Expense tracking

### 🔐 Security & Authentication
- Blockchain-based authentication
- Role-based access control
- Transaction transparency
- Immutable audit trails

## 🏗️ Architecture
Frontend (React PWA) ↔ Soroban RPC ↔ Stellar Blockchain ↔ 13 Smart Contracts

text

### Smart Contracts Deployed on Stellar Testnet

| Contract | Function | Testnet Address |
|----------|----------|-----------------|
| **analytics** | Data analysis & reporting | `CBMKG3CJ2IO25O3OVMUUGYUJ5WGRHXIK6OO5RFKGPPVPRZAWIB3N4DDU` |
| **crm_system** | Customer relationship management | `CBX4JC6DGKCUM5ZEZD3PG73TTPN5WH5RIT5LM2FT2BGYM3NDVYCJOBNE` |
| **pos_system** | Point of sale operations | `CDWIEF6SQTWBE2HAASKOYFOR7XZMSZGNOCGRSGWGCCMZ6MEBWRCNUBFL` |
| **xmoney_token** | Payment token ecosystem | `CBSPOOGIT7BRMA2AKVBYZJJJWFLAISOD5AFLZGOLDRLFTE24U6EKUBDW` |
| **auth_system** | Authentication & authorization | `CCXPTATFW7EUDTQDOGL43PVYJQXACFETGJX2PNMJW5FPMFXRUEQHBQBM` |
| **dashboard** | Real-time dashboard data | `CCL7U7Q5FN2RR2FLAH55UVX5XBRRYCGSSWN3GJVCBNCCCOAD5I54CUOM` |
| **accounting_system** | Financial management | `CBQ3GK2PKOWJYQOD2DUPYGAIOEV4ZPDWWLCJMZAI4BYSR6ODPALX2Q7Y` |
| **employee_manager** | Staff management | `CDHCADTSI6NX4IGZMCFHV45MQYG7HVLRVRIRC76YNYPRC6TJWZFBDQQJ` |
| **inventory** | Stock management | `CB5W5S7TV33UHC5IUNOQIE42BJPP6GZN3YUUFNDZOZ4GDSIZXPP243QI` |
| **loyalty** | Customer loyalty program | `CADHUT5YIQJ7XXDEHZ4S57ZZRUXXDV4PGZMOX67NMGO5CBI7WO6UFFHI` |
| **payment** | Payment processing | `CCUEZ64P5WLI7MGU3B6LFSYU2NYR4FMJZ3RVAJSQ46D5W42WSLUMH7YE` |
| **payroll** | Employee payroll | `CBP7ZDZNMUP76BSIUPO3KJZG4C6ULUCN77FI5EQTZSW5ZC22765GN5FM` |
| **vendor** | Supplier management | `CA3XT45HEJ5GVCIVDBVB6TFIURFLZFYRDKLFLPZ2ZMFTFBO2NIIYLE3U` |

## 🚀 Quick Start

### Prerequisites
- Node.js 18+ 
- Soroban CLI
- Freighter Wallet browser extension

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/retail-management-blockchain.git
cd retail-management-blockchain
Install dependencies

bash
npm install
Start development server

bash
npm run dev
Access the application
Open http://localhost:5173 and install as PWA desktop app.

Smart Contract Interaction
bash
# Initialize token contract
soroban contract invoke \
  --id <TOKEN_CONTRACT_ID> \
  --source <WALLET> \
  --network testnet \
  -- \
  initialize \
  --admin <ADMIN_ADDRESS> \
  --decimal 7 \
  --name "XMoney Token" \
  --symbol "XMT"
📱 PWA Features
✅ Install as desktop/mobile app

✅ Offline functionality

✅ Push notifications

✅ Cross-platform compatibility

✅ Automatic updates

✅ Secure blockchain transactions

🔧 Technology Stack
Frontend
React 18 - Modern UI framework

Vite - Fast build tool and dev server

TailwindCSS - Utility-first CSS framework

PWA - Progressive Web App capabilities

Blockchain & Smart Contracts
Stellar Soroban - Next-generation smart contracts platform

13 Smart Contracts - Comprehensive business logic

Rust - Contract development language

Freighter Wallet - User authentication and transaction signing

Development & Deployment
Soroban CLI - Contract management and deployment

Stellar Testnet - Development and testing network

GitHub Actions - CI/CD pipeline

🗂️ Project Structure
text
retail-management-blockchain/
├── contracts/                 # Smart contracts source code
│   ├── analytics/            # Sales data analysis system
│   ├── pos-system/           # Point of sale operations
│   ├── inventory/            # Inventory management logic
│   ├── crm-system/           # Customer relationship management
│   ├── token/                # XMoney token implementation
│   └── ... 8 more contracts
├── frontend/                 # React PWA application
│   ├── src/
│   │   ├── components/       # React components
│   │   │   ├── POSSystem/    # Point of sale interface
│   │   │   ├── Dashboard/    # Analytics dashboard
│   │   │   ├── Inventory/    # Stock management
│   │   │   └── Common/       # Shared components
│   │   ├── services/         # Blockchain integration
│   │   │   ├── soroban.js    # Soroban RPC configuration
│   │   │   ├── contracts.js  # Contract interaction layer
│   │   │   └── wallet.js     # Freighter wallet integration
│   │   ├── hooks/            # Custom React hooks
│   │   │   ├── useFreighter.js # Wallet connection hook
│   │   │   └── useContracts.js # Contract interaction hook
│   │   ├── utils/            # Helper functions
│   │   └── types/            # TypeScript definitions
│   ├── public/              # PWA assets and manifest
│   └── vite.config.js       # Build configuration
├── docs/                    # Documentation
├── scripts/                 # Deployment and utility scripts
└── tests/                   # Test suites
📊 System Workflow
Sales Process
Customer Selection → Scan products or manual entry

Cart Management → Real-time price calculation

Payment Processing → Multiple payment options

Blockchain Transaction → Immutable record creation

Inventory Update → Automatic stock deduction

Loyalty Points → Automatic rewards calculation

Inventory Management
Stock Monitoring → Real-time quantity tracking

Replenishment Alerts → Automated low stock notifications

Supplier Coordination → Automated purchase orders

Batch Tracking → Expiry and lot management

🔐 Security Features
Blockchain Immutability - All transactions permanently recorded

Role-based Access Control - Different permissions for staff/manager/admin

Encrypted Data - Sensitive information protection

Audit Trail - Complete transaction history

Multi-signature Support - Enhanced security for critical operations

🌐 Deployment
Smart Contracts (Testnet)
All 13 contracts successfully deployed and verified on Stellar Testnet.

Frontend Deployment
bash
# Build for production
npm run build

# Deploy to hosting service (Netlify/Vercel)
npm run deploy
🤝 Contributing
We welcome contributions from the community! Please see our Contributing Guide for details.

Fork the repository

Create your feature branch (git checkout -b feature/AmazingFeature)

Commit your changes (git commit -m 'Add some AmazingFeature')

Push to the branch (git push origin feature/AmazingFeature)

Open a Pull Request

📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

🙏 Acknowledgments
Stellar Development Foundation - For the amazing Soroban platform

Soroban Team - Continuous innovation in smart contracts

Freighter Wallet Team - Seamless user experience

Community Contributors - Valuable feedback and support

📞 Support & Contact
Documentation: Project Wiki

Issues: GitHub Issues

Discussions: GitHub Discussions

Email:minhtran1122006@gmail.com

🚀 Future Roadmap
Mobile app development

Advanced analytics with machine learning

Integration with external payment gateways

Multi-store support

API for third-party integrations

Advanced reporting features

Internationalization (i18n)

Built with ❤️ using Stellar Soroban - Revolutionizing Retail with Blockchain Technology
