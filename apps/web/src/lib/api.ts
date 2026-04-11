const DEFAULT_API_BASE_URL = '/api'

type ApiErrorPayload = {
  code?: string
  message?: string
}

export type AuthSessionResponse = {
  authenticated: boolean
  username?: string
}

export type ApiErrorCode =
  | 'invalid_credentials'
  | 'password_mismatch'
  | 'invalid_current_password'
  | 'invalid_token'
  | 'unauthenticated'
  | 'invalid_input'
  | 'internal_error'
  | string

export class ApiClientError extends Error {
  status: number
  code?: ApiErrorCode

  constructor(status: number, message: string, code?: ApiErrorCode) {
    super(message)
    this.name = 'ApiClientError'
    this.status = status
    this.code = code
  }
}

export const apiBaseUrl = removeTrailingSlash(
  import.meta.env.VITE_API_BASE_URL ?? DEFAULT_API_BASE_URL,
)

export async function getSession(signal?: AbortSignal): Promise<AuthSessionResponse> {
  return requestJson<AuthSessionResponse>('/auth/session', {
    method: 'GET',
    signal,
  })
}

export async function login(payload: {
  username: string
  password: string
}): Promise<void> {
  await request('/auth/login', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export async function logout(): Promise<void> {
  await request('/auth/logout', {
    method: 'POST',
  })
}

export async function changePassword(payload: {
  current_password: string
  new_password: string
  new_password_check: string
}): Promise<void> {
  await request('/admin/password', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export async function publishNewsletter(payload: {
  title: string
  text_content: string
  html_content: string
  idempotency_key: string
}): Promise<{ status: 'accepted' }> {
  return requestJson('/admin/newsletters', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export async function subscribe(payload: {
  name: string
  email: string
}): Promise<{ status: 'accepted' }> {
  return requestJson('/subscriptions', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export async function confirmSubscription(
  subscriptionToken: string,
  signal?: AbortSignal,
): Promise<{ status: 'confirmed' }> {
  return requestJson(`/subscriptions/confirm?subscription_token=${encodeURIComponent(subscriptionToken)}`, {
    method: 'GET',
    signal,
  })
}

async function requestJson<T>(
  path: string,
  init: RequestInit = {},
): Promise<T> {
  const response = await request(path, init)
  const hasBody = response.status !== 204
  if (!hasBody) {
    return {} as T
  }
  return (await response.json()) as T
}

async function request(path: string, init: RequestInit = {}): Promise<Response> {
  const response = await fetch(buildApiUrl(path), {
    ...init,
    credentials: 'include',
    headers: {
      Accept: 'application/json',
      'Content-Type': 'application/json',
      ...(init.headers ?? {}),
    },
  })

  if (response.ok) {
    return response
  }

  const errorPayload = await parseApiErrorPayload(response)
  throw new ApiClientError(
    response.status,
    errorPayload.message ?? `Request failed with status ${response.status}`,
    errorPayload.code,
  )
}

function buildApiUrl(path: string): string {
  const normalizedPath = path.startsWith('/') ? path : `/${path}`
  return `${apiBaseUrl}${normalizedPath}`
}

async function parseApiErrorPayload(response: Response): Promise<ApiErrorPayload> {
  const contentType = response.headers.get('content-type')
  if (contentType?.includes('application/json')) {
    return (await response.json()) as ApiErrorPayload
  }
  return {}
}

function removeTrailingSlash(value: string): string {
  if (value === '/') {
    return ''
  }
  return value.endsWith('/') ? value.slice(0, -1) : value
}
