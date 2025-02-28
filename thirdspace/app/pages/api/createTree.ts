import { NextApiRequest, NextApiResponse } from "next";
import { createBubblegumTree } from "../../lib/bubblegum";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "POST")
    return res.status(405).json({ error: "Method Not Allowed" });

  try {
    const treeAddress = await createBubblegumTree();
    return res.status(200).json({ treeAddress });
  } catch (error) {
    return res.status(500).json({ error: error.message });
  }
}
