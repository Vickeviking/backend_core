import { useState, type FormEvent } from 'react'
import { useNavigate } from 'react-router-dom'
import { ApiClientError, login } from '../lib/api'

export function LoginPage({
  onLoginSuccess,
}: {
  onLoginSuccess: () => Promise<void>
}) {
  const navigate = useNavigate()
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [errorMessage, setErrorMessage] = useState<string | null>(null)

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setIsSubmitting(true)
    setErrorMessage(null)

    try {
      await login({ username, password })
      await onLoginSuccess()
      navigate('/admin/dashboard', { replace: true })
    } catch (requestError) {
      if (
        requestError instanceof ApiClientError &&
        requestError.code === 'invalid_credentials'
      ) {
        setErrorMessage('Invalid username or password.')
      } else {
        setErrorMessage(
          requestError instanceof Error
            ? requestError.message
            : 'Login failed unexpectedly.',
        )
      }
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <section className="page-card narrow-card">
      <p className="panel-title">Admin</p>
      <h2>Sign In</h2>
      <p>Use your admin credentials to access password and newsletter operations.</p>

      <form className="stack-form" onSubmit={onSubmit}>
        <label>
          Username
          <input
            value={username}
            onChange={(event) => {
              setUsername(event.target.value)
            }}
            autoComplete="username"
            required
          />
        </label>

        <label>
          Password
          <input
            type="password"
            value={password}
            onChange={(event) => {
              setPassword(event.target.value)
            }}
            autoComplete="current-password"
            required
          />
        </label>

        <button className="primary-button" type="submit" disabled={isSubmitting}>
          {isSubmitting ? 'Signing in...' : 'Sign in'}
        </button>
      </form>

      {errorMessage ? <p className="status-note status-note-error">{errorMessage}</p> : null}
    </section>
  )
}
