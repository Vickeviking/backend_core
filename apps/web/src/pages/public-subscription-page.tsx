import { useState, type FormEvent } from 'react'
import { ApiClientError, subscribe } from '../lib/api'

export function PublicSubscriptionPage() {
  const [name, setName] = useState('')
  const [email, setEmail] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [feedback, setFeedback] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setIsSubmitting(true)
    setFeedback(null)
    setError(null)

    try {
      await subscribe({ name, email })
      setFeedback('Subscription request accepted. Check your inbox to confirm.')
      setName('')
      setEmail('')
    } catch (requestError) {
      if (requestError instanceof ApiClientError && requestError.code === 'invalid_input') {
        setError('Please provide a valid name and email address.')
      } else {
        setError(
          requestError instanceof Error
            ? requestError.message
            : 'Subscription failed unexpectedly.',
        )
      }
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <section className="page-card">
      <p className="panel-title">Public</p>
      <h2>Join The Newsletter</h2>
      <p>
        Enter your details and we will send a confirmation link. Your address is only
        activated after confirmation.
      </p>

      <form className="stack-form" onSubmit={onSubmit}>
        <label>
          Name
          <input
            value={name}
            onChange={(event) => {
              setName(event.target.value)
            }}
            required
            autoComplete="name"
          />
        </label>

        <label>
          Email
          <input
            type="email"
            value={email}
            onChange={(event) => {
              setEmail(event.target.value)
            }}
            required
            autoComplete="email"
          />
        </label>

        <button className="primary-button" type="submit" disabled={isSubmitting}>
          {isSubmitting ? 'Submitting...' : 'Subscribe'}
        </button>
      </form>

      {feedback ? <p className="status-note status-note-ok">{feedback}</p> : null}
      {error ? <p className="status-note status-note-error">{error}</p> : null}
    </section>
  )
}
