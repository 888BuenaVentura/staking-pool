import { Connection, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { createHash } from "crypto";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";

async function main() {
  const connection = new Connection("https://api.devnet.solana.com");
  const keypairPath = path.join(os.homedir(), ".config/solana/id.json");
  const secretKey = Buffer.from(JSON.parse(fs.readFileSync(keypairPath)));
  const wallet = Keypair.fromSecretKey(secretKey);
  const programId = new PublicKey("8f6ZoJv8UeBsY7khEKX7ddhcjt2JLhKbHQdRnCBZ3mYa");

  const pool = Keypair.generate();

  const discriminator = createHash("sha256").update("global:initialize_pool").digest().subarray(0, 8);
  const rewardRate = 1000000n;
  const data = Buffer.concat([discriminator, Buffer.from(rewardRate.toString(16).padStart(16, "0"), "hex")]);

  const initIx = new TransactionInstruction({
    programId,
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: pool.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data,
  });

  const tx = new Transaction().add(initIx);
  const sig = await connection.sendTransaction(tx, [wallet, pool]);
  await connection.confirmTransaction(sig);

  console.log("TX Signature:", sig);
  console.log("Pool Account:", pool.publicKey.toString());
  console.log("Explorer: https://explorer.solana.com/tx/" + sig + "?cluster=devnet");
}

main().catch(e => { console.error(e); process.exit(1); });
