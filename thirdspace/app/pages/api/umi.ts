import { NextApiRequest, NextApiResponse } from "next";
import { getUmiFromFile } from "../../lib/umi";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "GET")
    return res.status(405).json({ error: "Method Not Allowed" });

  try {
    const umi = getUmiFromFile();
    console.log("Fetched UMI obj: ", umi);
    return res.status(200).json({ umi });
  } catch (error) {
    return res.status(500).json({ error: error.message });
  }
}
