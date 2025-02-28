import { NextApiRequest, NextApiResponse } from "next";
import { updateCNFT } from "../../../lib/updateCNFT";

/**************************************************
Example API Call:
curl -X PUT "http://localhost:3000/api/update_cnft/update_cnft?asset_id=AwuMq9MLumGCbqGAe91JFffXuqQP2G3j8C8oMtFEqkiW&owner_id=BBkGcjr14MDp5qQQgEfsJ9bXPMW2Qy5Fpd8Gpc8HbAAm" \
     -H "Content-Type: application/json"

***************************************************/

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "PUT") {
    return res.status(405).json({ error: "Method Not Allowed" });
  }

  try {
    // ✅ Ensure request body is parsed correctly
    const assetId = req.query.asset_id as string;
    const ownerId = req.query.owner_id as string;
    console.log("Asset ID:", assetId);

    // ✅ Validate assetId
    if (!assetId || typeof assetId !== "string" || assetId.trim() === "") {
      return res
        .status(400)
        .json({ error: "❌ Invalid or missing asset_id parameter" });
    }

    // ✅ Validate ownerId
    if (!ownerId || typeof ownerId !== "string" || ownerId.trim() === "") {
      return res
        .status(400)
        .json({ error: "❌ Invalid or missing owner_id parameter" });
    }

    const collectionNFT = await updateCNFT(assetId, ownerId);
    return res.status(200).json({ success: true, collectionNFT });
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error minting CNFT:", errorMessage);
    return res.status(500).json({ error: errorMessage });
  }
}
