import { useState, type FormEvent } from 'react'
import { ApiClientError, publishNewsletter } from '../lib/api'

export function AdminNewslettersPage() {
  const [title, setTitle] = useState('')
  const [textContent, setTextContent] = useState('')
  const [htmlContent, setHtmlContent] = useState('')
  const [idempotencyKey, setIdempotencyKey] = useState<string>(crypto.randomUUID())
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [errorMessage, setErrorMessage] = useState<string | null>(null)
  const [successMessage, setSuccessMessage] = useState<string | null>(null)

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setIsSubmitting(true)
    setErrorMessage(null)
    setSuccessMessage(null)

    try {
      await publishNewsletter({
        title,
        text_content: textContent,
        html_content: htmlContent,
        idempotency_key: idempotencyKey,
      })

      setSuccessMessage('Issue accepted. Delivery worker will process the queue shortly.')
      setIdempotencyKey(crypto.randomUUID())
    } catch (requestError) {
      if (
        requestError instanceof ApiClientError &&
        requestError.code === 'unauthenticated'
      ) {
        setErrorMessage('Session expired. Please sign in again.')
      } else {
        setErrorMessage(
          requestError instanceof Error
            ? requestError.message
            : 'Failed to publish newsletter.',
        )
      }
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <section className="page-card">
      <p className="panel-title">Editorial</p>
      <h2>Publish Newsletter</h2>
      <p>
        This submits one issue and queues deliveries for all confirmed subscribers. Keep the
        idempotency key stable when retrying failed requests.
      </p>

      <form className="stack-form" onSubmit={onSubmit}>
        <label>
          Title
          <input
            value={title}
            onChange={(event) => {
              setTitle(event.target.value)
            }}
            required
          />
        </label>

        <label>
          Plain text content
          <textarea
            value={textContent}
            onChange={(event) => {
              setTextContent(event.target.value)
            }}
            rows={8}
            required
          />
        </label>

        <label>
          HTML content
          <textarea
            value={htmlContent}
            onChange={(event) => {
              setHtmlContent(event.target.value)
            }}
            rows={8}
            required
          />
        </label>

        <label>
          Idempotency key
          <input
            value={idempotencyKey}
            onChange={(event) => {
              setIdempotencyKey(event.target.value)
            }}
            required
          />
        </label>

        <button className="primary-button" type="submit" disabled={isSubmitting}>
          {isSubmitting ? 'Publishing...' : 'Publish issue'}
        </button>
      </form>

      {successMessage ? <p className="status-note status-note-ok">{successMessage}</p> : null}
      {errorMessage ? <p className="status-note status-note-error">{errorMessage}</p> : null}
    </section>
  )
}
