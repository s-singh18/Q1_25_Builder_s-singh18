import { NextApiRequest, NextApiResponse } from "next";
import { fetchBubblegumTreeConfig } from "../../lib/bubblegum";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "GET")
    return res.status(405).json({ error: "Method Not Allowed" });

  const treeAddress = Array.isArray(req.query.treeAddress)
    ? req.query.treeAddress[0] // Take the first value if it's an array
    : req.query.treeAddress;

  if (!treeAddress)
    return res.status(400).json({ error: "Missing tree address" });

  try {
    const treeData = await fetchBubblegumTreeConfig(treeAddress);
    return res.status(200).json(treeData);
  } catch (error) {
    return res.status(500).json({ error: error.message });
  }
}
