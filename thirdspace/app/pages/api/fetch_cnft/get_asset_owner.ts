import { NextApiRequest, NextApiResponse } from "next";
import { getAssetByOwner } from "../../../lib/fetchCNFT";

/**************************************************
Example API Call:
curl -X GET \
"http://localhost:3000/api/fetch_cnft/get_asset_owner \
?owner_id=BBkGcjr14MDp5qQQgEfsJ9bXPMW2Qy5Fpd8Gpc8HbAAm"
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

    const ownerId: string = req.query.owner_id as string;

    console.log("Creator ID:", ownerId);

    // ✅ Validate assetId
    if (!ownerId || typeof ownerId !== "string" || ownerId.trim() === "") {
      return res
        .status(400)
        .json({ error: "❌ Invalid or missing authority parameter" });
    }

    const collectionNFT = await getAssetByOwner({
      ownerId,
    });
    return res.status(200).json({ success: true, collectionNFT });
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error minting CNFT:", errorMessage);
    return res.status(500).json({ error: errorMessage });
  }
}
