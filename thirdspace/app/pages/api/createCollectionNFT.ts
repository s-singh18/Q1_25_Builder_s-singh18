import { NextApiRequest, NextApiResponse } from "next";
import { createCollectionNFT } from "../../lib/mintCNFT";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method Not Allowed" });
  }

  try {
    // ✅ Ensure request body is parsed correctly
    /*
    const { leafOwnerAddr, merkleTreeAddr } = req.body ?? {};

    if (!leafOwnerAddr || !merkleTreeAddr) {
      return res.status(400).json({
        error: "Missing required parameters: leafOwnerAddr, merkleTreeAddr",
      });
    }
    */
    // leafOwnerAddr, merkleTreeAddr
    const collectionNFT = await createCollectionNFT();
    return res.status(200).json({ success: true, collectionNFT });
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error minting CNFT:", errorMessage);
    return res.status(500).json({ error: errorMessage });
  }
}
