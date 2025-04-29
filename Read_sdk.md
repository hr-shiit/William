# Stellar Payment SDK

A simple Python implementation for making payments on the Stellar network.

## Features

- Send XLM payments on Stellar testnet
- Simple command-line interface
- Easy to use API

## Setup

1. Install dependencies:
```bash
pip install stellar-sdk
```

2. Configure your keys in `config.py`:
- Set your testnet secret key in `SOURCE_SECRET`
- Set your testnet public key in `MONITOR_ACCOUNT_ID`

## Usage

Run the payment agent:
```bash
python agent.py
```

Follow the prompts to:
1. Enter destination public key
2. Enter amount to send in XLM

## Files

- `agent.py`: Command-line interface for making payments
- `payments.py`: Core payment functionality using Stellar SDK
- `config.py`: Configuration settings and keys
- `horizon.py`: Utility functions for interacting with Horizon API
