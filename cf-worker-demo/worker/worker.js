addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const url = new URL(request.url)
  const path = url.pathname
  const { router } = wasm_bindgen;
  await wasm_bindgen(wasm)
  const content = router(path)
  return new Response(content, {status: 200, headers: {'content-type': 'text/html;charset=UTF-8'}})
}
