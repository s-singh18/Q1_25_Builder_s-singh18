import { NextApiRequest, NextApiResponse } from "next";
import { getAssetProof } from "../../../lib/fetchCNFT";

/**************************************************
Example API Call:
curl -X GET "http://localhost:3000/api/fetch_cnft/get_asset_proof?asset_id=AwuMq9MLumGCbqGAe91JFffXuqQP2G3j8C8oMtFEqkiW"
***************************************************/

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "GET") {
    return res.status(405).json({ error: "Method Not Allowed" });
  }

  try {
    // ✅ Ensure request body is parsed correctly

    const assetId = req.query.asset_id as string;
    console.log("Asset ID:", assetId);

    // ✅ Validate assetId
    if (!assetId || typeof assetId !== "string" || assetId.trim() === "") {
      return res
        .status(400)
        .json({ error: "❌ Invalid or missing asset_id parameter" });
    }

    const collectionNFT = await getAssetProof(assetId);
    return res.status(200).json({ success: true, collectionNFT });
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error minting CNFT:", errorMessage);
    return res.status(500).json({ error: errorMessage });
  }
}
