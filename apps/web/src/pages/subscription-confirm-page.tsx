import { useEffect, useState } from 'react'
import { useSearchParams } from 'react-router-dom'
import { ApiClientError, confirmSubscription } from '../lib/api'

export function SubscriptionConfirmPage() {
  const [searchParams] = useSearchParams()
  const token = searchParams.get('subscription_token')
  const [status, setStatus] = useState<'pending' | 'confirmed' | 'error'>(
    token ? 'pending' : 'error',
  )
  const [errorMessage, setErrorMessage] = useState<string | null>(
    token ? null : 'Missing subscription token in the URL.',
  )

  useEffect(() => {
    if (!token) {
      return
    }

    const controller = new AbortController()

    void (async () => {
      try {
        await confirmSubscription(token, controller.signal)
        setStatus('confirmed')
      } catch (requestError) {
        if (controller.signal.aborted) {
          return
        }

        setStatus('error')
        if (
          requestError instanceof ApiClientError &&
          requestError.code === 'invalid_token'
        ) {
          setErrorMessage('This confirmation link is invalid or expired.')
        } else {
          setErrorMessage(
            requestError instanceof Error
              ? requestError.message
              : 'Unable to confirm subscription.',
          )
        }
      }
    })()

    return () => {
      controller.abort()
    }
  }, [token])

  return (
    <section className="page-card narrow-card">
      <p className="panel-title">Confirmation</p>
      <h2>Subscription Confirmation</h2>

      {status === 'pending' ? (
        <p>Confirming your subscription...</p>
      ) : null}

      {status === 'confirmed' ? (
        <p className="status-note status-note-ok">
          Your subscription is confirmed. You are on the list.
        </p>
      ) : null}

      {status === 'error' ? (
        <p className="status-note status-note-error">{errorMessage}</p>
      ) : null}
    </section>
  )
}
