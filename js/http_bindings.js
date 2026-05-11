async function wasm_fetch_with_request(url, options) {
  const method = options?.method ?? "GET";
  const headers = options?.headers ?? {};
  const body = options?.body;

  return fetch(url, {
    method,
    headers,
    body,
  });
}

async function response_headers(response) {
  if (!(response instanceof Response)) {
    throw new Error("Input is not a Response object");
  }

  const headers = {};
  for (const [key, value] of response.headers.entries()) {
    headers[key] = value;
  }
  return headers;
}

async function response_text(response) {
  if (!(response instanceof Response)) {
    throw new Error("Input is not a Response object");
  }
  return response.text();
}

async function response_status(response) {
  if (!(response instanceof Response)) {
    throw new Error("Input is not a Response object");
  }
  return response.status;
}

export {
  wasm_fetch_with_request,
  response_headers,
  response_text,
  response_status,
};
