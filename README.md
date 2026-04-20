# Stellar Student Inventory DApp

**Stellar Student Inventory DApp** - Blockchain-Based Student Inventory Management System

## Project Description

Stellar Student Inventory DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure, immutable platform for managing student inventory items directly on the blockchain. The contract ensures that your data is stored transparently and is only manageable through predefined smart contract functions, eliminating reliance on centralized database providers.

The system allows users to add, view, update, and delete inventory items, leveraging the efficiency and security of the Stellar network. Each item is uniquely identified and stored within the contract's instance storage, ensuring data persistence and reliability.

## Project Vision

Our vision is to modernize asset management in academic environments by:

- **Decentralizing Data**: Moving inventory records from centralized systems to a global, distributed blockchain.
- **Ensuring Ownership**: Empowering users to have complete control and ownership over their inventory data.
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of items that cannot be altered or deleted by third parties.
- **Enhancing Transparency**: Leveraging blockchain security to make all inventory activities verifiable.
- **Building Trustless Systems**: Creating a platform where data integrity is guaranteed by code, not by company promises.

## Key Features

### 1. Simple Item Creation
- Create inventory items with just one function call.
- Specify item name, quantity, and owner.
- Automated ID generation for unique identification.

### 2. Efficient Data Retrieval
- Fetch all stored items in a single call.
- Structured data representation for easy frontend integration.
- Real-time synchronization with the blockchain state.

### 3. Secure Deletion & Updates
- Remove specific items using their unique IDs.
- Update item quantity (`update_jumlah`) to reflect real-time stock changes.
- Permanent removal/update from the contract storage.

### 4. Stellar Network Integration
- Built using the modern **Soroban Smart Contract SDK**.
- Leverages high speed and low cost of the Stellar network.
- Uses instance storage for data persistence.

## Contract Details

- **Contract Address**: `CCTQNYQGZSBLHWRZIWQ6DS4GUB4DV5N2SU6C23AOX5XUAJK4U3TD3TR4`

### Screenshot Preview
![Main Dashboard Interface](https://via.placeholder.com/800x400.png?text=Upload+Your+Screenshot+Here)
*Ganti URL di atas dengan cara drag-and-drop gambar Anda langsung ke editor GitHub.*

---

## Technical Requirements

- **Language**: Rust
- **SDK**: Soroban SDK
- **Network**: Stellar Testnet/Futurenet

## Getting Started

Interact with the smart contract using these main functions:

1.  `add_item()`: Add a new inventory item.
2.  `get_items()`: Retrieve all stored items.
3.  `delete_item()`: Remove a specific item by its ID.
4.  `update_jumlah()`: Update the quantity of an item.

---

## Future Scope

- **Short-Term**: Borrow & Return System, Category Management, and Search Filters.
- **Medium-Term**: Access Control (RBAC), Shared Inventory, and Activity Tracking.
- **Long-Term**: DAO Governance, Cross-Chain Sync, and Mobile Optimization.

---

**Stellar Student Inventory DApp** - Managing Student Assets Securely on the Blockchain 🚀