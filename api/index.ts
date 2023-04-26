/* @ts-ignore */
import wasm from "../wasm/pkg/vercel_wasm_bg.wasm?module";
import init, { gen } from "../wasm/pkg/vercel_wasm.js";

export const config = {
  runtime: "edge",
};

export default async function handler(request: Request, event: Event) {
  await init(wasm);

  const value = gen();
  const resp = new Response(value);
  resp.headers.set("content-type", "text-html");

  return resp;
}
