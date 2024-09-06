// FOR DEVELOPMENT CONNECTION TO API

import { createProxyMiddleware } from 'http-proxy-middleware'
import { NextApiRequest, NextApiResponse } from 'next'

const proxy = createProxyMiddleware({
  target: 'http://127.0.0.1:9099',
})

export default function handler(req: NextApiRequest, res: NextApiResponse) {
  console.log('Proxying request to:', req.url);
  // Don't allow proxy in production
  if (process.env.NODE_ENV === 'production') {
    return res.status(404).json({ message: 'Not AFASFFound' })
  }
  // @ts-ignore
  proxy(req, res, (result: unknown) => {
    if (result instanceof Error) {
      throw result
    }
    throw new Error(`Request '${req.url}' is not proxied! We should never reach here!`)
  })
}

export const config = {
  api: {
    bodyParser: false,
  },
}
