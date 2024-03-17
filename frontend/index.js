const { Connection, PublicKey, Transaction, TransactionInstruction } = require('@solana/web3.js');

async function callContract() {
    // Initialize connection to Solana network
    const connection = new Connection('https://api.mainnet-beta.solana.com');

    // Load the smart contract program
    const programId = new PublicKey('your_contract_program_id');

    // Create a transaction instruction to call the contract function
    const instruction = new TransactionInstruction({
        keys: [{pubkey: new PublicKey('your_account_pubkey'), isSigner: true, isWritable: true}],
        programId,
        data: Buffer.from([0, 1, 2, 3]) // Encode function arguments as needed
    });

    // Build the transaction
    const transaction = new Transaction().add(instruction);

    // Sign and send the transaction
    const privateKey = Buffer.from('your_private_key', 'hex'); // Private key of the account sending the transaction
    const signature = await connection.sendTransaction(transaction, [privateKey]);

    // Wait for confirmation
    await connection.confirmTransaction(signature);
    console.log('Transaction confirmed:', signature);
}

callContract();
