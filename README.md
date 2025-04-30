William: 





A Decentralized InHeritance In Stellar





BY 




HARSHIT SRIVASTAV


TEJASVI



RUPALI CHHAYA






Video Limk


https://www.loom.com/share/b3c25b7abc3b4ada937e227a97a35a74?sid=076ad255-08d6-419a-92f4-9b6c0677cbe5



MVP DEMO
https://stellar-anchor-weaver.lovable.app

For Working process check WIKI.

Abstract


The William Web3 wallet introduces a novel approach to posthumous digital asset management by integrating a blockchain-based smart contract with an AI-driven death verification system. Deployed on the Soroban blockchain, the system employs a three-step verification process—user activity monitoring, nominee voting, and AI-based analysis of death certificates and digital footprints—to confirm a user’s death before distributing funds to designated beneficiaries. A Python automation script facilitates email-based reminders and voting, while a robust smart contract ensures secure fund management. This paper presents the system’s technical architecture, evaluates its design through rigorous analysis, and discusses its implications for decentralized finance (DeFi) and digital legacy management. Despite its innovative design, challenges such as data access, email security, and ethical concerns warrant further exploration.

PROJECT DESCRIPTION


The William Web3 Wallet is a decentralized digital estate management system deployed on the STELLAR Blockchain. It ensures secure, automated transfer of digital assets after a user’s death through a three-step verification process. First, a Python script monitors user inactivity and sends reminder emails. If unresponsive, nominees vote on the user’s death. If the majority confirms, an AI system analyzes submitted death certificates and digital footprints to verify death. Once confirmed, a Soroban smart contract distributes funds to predefined beneficiaries, with security, transparency, and no intermediaries. Designed for integration into custodial wallets, the system includes modular components (smart contract, Python script, AI SDK) and prioritizes privacy and automation. While innovative, it faces challenges in data access, security, and automation gaps, which future updates aim to address. The William Wallet offers a scalable, trustless solution for managing digital legacies in Web3.



PROJECT VISION




The William Web3 Wallet envisions a future where digital assets are securely passed on without lawyers, courts, or centralized services. By combining blockchain, AI, and automation, it creates a trustless, transparent system for managing digital legacies after death. Designed as an SDK, William can be open-sourced and easily integrated into other wallets, allowing widespread adoption of posthumous asset management. This empowers individuals with control over their assets, reduces stress for families, and ensures fair, automatic distribution. As Web3 grows, William aims to set the standard for secure, decentralized estate planning across platforms, borders, and technologies.










WORKING PROGRESS AND CODE BASE DETAIL.

1. Introduction
Web3 technologies, powered by decentralized blockchain networks, enable secure and transparent management of digital assets. However, managing these assets after a user’s death remains a significant challenge, as traditional estate planning relies on centralized intermediaries incompatible with Web3’s trustless ethos. The William Web3 wallet addresses this gap by combining a Soroban-based smart contract with an AI-driven death verification system and Python automation for user and nominee interactions. The system verifies death through a three-step process—monitoring user inactivity, collecting nominee votes, and analyzing death certificates and digital footprints—before automatically distributing funds via smart contracts. This paper describes the system’s architecture, analyzes its technical and ethical dimensions, and evaluates its potential to redefine digital legacy management.
2. Related Work
Existing solutions for posthumous asset management in blockchain ecosystems are limited. Custodial wallets often rely on manual executor processes, introducing centralization and trust dependencies (Swan, 2015). Smart contract-based wills, such as those proposed by Ethereum developers (Wood, 2014), enable automated fund distribution but lack robust death verification mechanisms. AI applications in identity verification, such as KYC/AML systems (Fatoum et al., 2021), demonstrate the potential for automated analysis but are rarely applied to death confirmation. The William system bridges these gaps by integrating AI-driven verification with a decentralized smart contract, offering a fully automated and trustless solution. Unlike prior systems, it combines human input (nominee voting) with AI analysis, addressing both reliability and scalability.
3. Technical Architecture
The William system comprises four core components: a Soroban smart contract, a Python automation script for email interactions, an AI SDK for death verification (assumed, not provided), and a distribution SDK (agent.py, payment.py, referenced but not provided). The architecture is modular, with each component handling distinct aspects of the death verification and fund distribution process.
3.1 Soroban Smart Contract
The WilliamContract, implemented in Rust for the Soroban blockchain, manages the wallet’s core logic. Key features include:
State Management: Stores the owner’s address, beneficiary addresses with percentage allocations, total deposited funds, and a death status flag (IsDead) using persistent storage (DataKey enum).

Functions:
initialize: Sets the owner and initializes state (empty beneficiary list, zero balance, IsDead = false).

store_beneficiary/delete_beneficiary: Manages beneficiaries, ensuring percentages are between 1 and 100.

deposit_core: Tracks fund deposits (assumes token client integration for actual transfers).

death_check: Updates the IsDead status, restricted to the owner (assumed to interface with AI verification).

ben_transfer: Distributes funds to beneficiaries if IsDead = true, ensuring total percentages sum to 100.

View functions (e.g., get_owner, get_contract_balance) for querying state.

Event Emissions: Logs actions (e.g., beneficiary added, funds deposited) for transparency.

Testing: A test suite (test.rs) validates initialization, beneficiary management, and death status updates, ensuring robustness.

The contract operates in a #![no_std] environment, optimizing for blockchain efficiency. However, it assumes external token transfers and manual death_check calls, suggesting integration with the AI SDK.
3.2 Python Automation Script
The email_automation.py script handles email-based interactions for the first two verification steps:
Activity Monitoring: Checks user login activity every 24 hours using a SQLite database. If a user is inactive for 180 days, it initiates a 60-day reminder phase, sending up to eight weekly emails.

Nominee Voting: If reminders go unanswered, the script sends voting emails to nominees, providing a secure URL to vote on the user’s death status. A >50% “dead” vote triggers the next verification step.

Database: Stores user data (address, email, last login, reminder status) and nominee data (email, vote status).

Security: Uses TLS for email transmission and assumes secure voting URLs (e.g., HTTPS).

The script is extensible, supporting API integration for vote submission and certificate uploads, though it requires secure credential management.
3.3 AI Verification SDK
The AI SDK (not provided, assumed based on description) handles the third verification step:
Death Certificate Validation: Cross-references uploaded certificates with government datasets, assuming API access to public records.

Digital Footprint Analysis: Scrapes social media and digital platforms (e.g., Twitter, LinkedIn) from the presumed death date, using natural language processing (NLP) to detect inactivity or memorialization (e.g., condolence posts).

Output: Confirms death if both sub-steps are satisfied, triggering a call to the smart contract’s death_check function (assumed to be automated via an off-chain client).

The SDK’s absence in the repository limits analysis, but its integration is critical for automation.
3.4 Distribution SDK
The agent.py and payment.py files (referenced but not provided) are assumed to interface with the smart contract’s ben_transfer function, executing token transfers to beneficiaries. They likely use a Soroban client to interact with the blockchain, ensuring funds are distributed according to the contract’s logic.
3.5 System Workflow
The system operates as follows (see Figure 1):
Activity Monitoring: The Python script monitors user logins via the database. After 180 days of inactivity, it sends reminders for 60 days.

Nominee Voting: If no response, the script emails nominees to vote. A >50% “dead” vote initiates certificate verification.

AI Verification: Nominees upload a death certificate, and the AI SDK validates it and analyzes digital footprints. If confirmed, it updates the smart contract’s IsDead status.

Fund Distribution: The smart contract’s ben_transfer function disburses funds to beneficiaries, with the distribution SDK handling token transfers.
![Uploading Screenshot 2025-04-30 at 12.49.45 PM.png…]()

Figure 1: System Workflow
[Placeholder: Include a flowchart depicting the three-step verification and distribution process.]
4. Methodology
The system’s methodology is formalized as a sequential process:
Activity Monitoring:
Input: User login timestamps (database).

Condition: No login for 180 days + no response to 60-day reminders (8 emails).

Output: Initiate nominee voting.

Nominee Voting:
Input: Binary votes (dead/alive) from nominees (via secure URL).

Condition: >50% “dead” votes.

Output: Trigger AI verification.

AI Verification:
Input: Death certificate, social media links.

Sub-processes:
Certificate validation (government dataset matching).

Digital footprint analysis (NLP-based inactivity detection).

Condition: Both sub-processes confirm death.

Output: Set IsDead = true in smart contract.

Fund Distribution:
Input: Beneficiary addresses, percentages (smart contract storage).

Process: Execute ben_transfer to distribute funds.

Output: Funds transferred, TotalDeposited reset to 0.

5. Evaluation
The William system is evaluated across technical, ethical, and practical dimensions, leveraging the smart contract, tests, and automation script.
5.1 Technical Strengths  
Decentralization: The Soroban smart contract eliminates intermediaries, ensuring trustless fund management. Its #![no_std] design optimizes for blockchain efficiency.

Robust Verification: The three-step process reduces false positives by combining passive monitoring, human input, and AI analysis.

Test Coverage: The test suite validates critical functions (initialization, beneficiary management, death status), ensuring reliability.

Automation: The Python script automates email interactions, enhancing scalability for large user bases.

Event Logging: Smart contract events provide transparency, enabling auditability of actions.

5.2 Technical Limitations  
Data Access: The AI SDK’s reliance on government datasets and social media APIs is problematic, as access varies by jurisdiction and platform (e.g., Twitter’s API restrictions post-2023).

Email Security: The Python script’s email-based reminders and voting are vulnerable to phishing, spam, or account compromise. HTTPS voting URLs mitigate but don’t eliminate risks.

Manual Death Check: The smart contract’s death_check function requires owner authentication, conflicting with the automated AI verification process. An off-chain client is needed to bridge this gap, but its implementation is unspecified.

Incomplete SDKs: The absence of the AI SDK and distribution SDK (agent.py, payment.py) limits transparency and reproducibility.

Token Integration: The smart contract assumes token transfers via a client, but the lack of explicit implementation introduces ambiguity.

Test Gaps: The test suite omits edge cases (e.g., ben_transfer with partial percentages, concurrent beneficiary updates), potentially missing vulnerabilities.


The William Web3 wallet demonstrates the potential of AI and blockchain to address posthumous asset management. Its Soroban smart contract, Python automation, and AI-driven verification provide a robust framework for decentralized legacy planning. However, technical gaps (e.g., incomplete SDKs, manual death_check), security risks (e.g., email vulnerabilities), and ethical challenges (e.g., privacy) necessitate further refinement. By addressing these, William could set a new standard for digital estate management in Web3 ecosystems.
9. Future Work  
Open-Source SDKs: Release the AI and distribution SDKs to enable community scrutiny and improvement.

Secure Automation: Implement OAuth2 for email, rate limiting for voting, and a robust database (e.g., PostgreSQL).

Automated Death Check: Replace manual death_check with an oracle or multi-signature mechanism to align with AI verification.

Privacy-Preserving AI: Use zero-knowledge proofs for certificate validation to protect user data.

Global Compatibility: Adapt verification for diverse jurisdictions and cultural contexts.





Future plan: We can create it as a Sdk that can be implimenteed into any costodiale wallet for account recovery function.



Enhanced Testing: Expand the test suite to cover edge cases (e.g., concurrent updates, partial percentages).



