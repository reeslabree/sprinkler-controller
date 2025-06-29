import { NextApiRequest, NextApiResponse } from "next";

interface Request {
  zone: number;
}

interface Response {}

const handler = async (req: NextApiRequest, res: NextApiResponse) => {
  const { zone } = req.body as Request;

  if (zone < 0 || zone > 6) {
    res.status(400).json({ error: "Invalid zone" });
    return;
  }
};

export default handler;
