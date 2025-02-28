import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  Umi,
} from "@metaplex-foundation/umi";
import wallet from "../Turbin3-wallet.json";
import {
  createTree,
  fetchMerkleTree,
  fetchTreeConfigFromSeeds,
} from "@metaplex-foundation/mpl-bubblegum";
import { publicKey } from "@metaplex-foundation/umi";

console.log("Running bubblegum tree script...");
// Initialize Solana connection and payer wallet.
const umi: Umi = createUmi("https://api.devnet.solana.com");

try {
  const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
  const signer = createSignerFromKeypair(umi, keypair);

  // Tell Umi to use the new signer.
  umi.use(signerIdentity(signer));
} catch (error) {
  console.error("❌ Error reading wallet file:", error.message);
}

// Keypairs saved as Uint8Array, transform to keypair.

/**
 * Create a new Bubblegum Merkle tree.
 */
export const createBubblegumTree = async () => {
  const maxDepth = 14; // 2^14 = ~16,384 leaves
  const maxBufferSize = 256;

  const merkleTree = generateSigner(umi);
  console.log("Merkle Tree:", merkleTree);

  const builder = await createTree(umi, {
    merkleTree,
    maxDepth: maxDepth,
    maxBufferSize: maxBufferSize,
    treeCreator: merkleTree,
    public: true,
  });

  await builder.sendAndConfirm(umi);

  console.log("Builder: ", builder);
  return builder;
};

/**
 * Fetches an existing Bubblegum tree.
 */
export const fetchBubblegumTree = async (treeAddress: string) => {
  // merkleTreeAddr: "CK6tUG2ktsuRignn5cKYTSqTAS1rs977hAcLT45AjDYk"
  const merkleTree = publicKey(treeAddress);

  try {
    const merkleTreeAccount = await fetchMerkleTree(umi, merkleTree);
    console.log("✅ Tree Found:", merkleTreeAccount);

    // ✅ Convert BigInt values to strings
    const serializedData = JSON.parse(
      JSON.stringify(merkleTreeAccount, (_, value) =>
        typeof value === "bigint" ? value.toString() : value
      )
    );

    return serializedData;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error fetching Merkle tree:", errorMessage);
    throw new Error(errorMessage);
  }
};

/**
 * Fetches an existing Bubblegum tree config.
 */
export const fetchBubblegumTreeConfig = async (treeAddress: string) => {
  const merkleTree = publicKey(treeAddress);

  try {
    const treeConfig = await fetchTreeConfigFromSeeds(umi, { merkleTree });
    console.log("✅ Tree Config Found:", treeConfig);
    // ✅ Convert BigInt values to strings
    const serializedData = JSON.parse(
      JSON.stringify(treeConfig, (_, value) =>
        typeof value === "bigint" ? value.toString() : value
      )
    );

    return serializedData;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error fetching Merkle tree config:", errorMessage);
    throw new Error(errorMessage);
  }
};
