import { useState, type FormEvent } from 'react'
import { ApiClientError, changePassword } from '../lib/api'

export function AdminPasswordPage() {
  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [newPasswordCheck, setNewPasswordCheck] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [errorMessage, setErrorMessage] = useState<string | null>(null)
  const [successMessage, setSuccessMessage] = useState<string | null>(null)

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setIsSubmitting(true)
    setErrorMessage(null)
    setSuccessMessage(null)

    try {
      await changePassword({
        current_password: currentPassword,
        new_password: newPassword,
        new_password_check: newPasswordCheck,
      })
      setCurrentPassword('')
      setNewPassword('')
      setNewPasswordCheck('')
      setSuccessMessage('Password updated successfully.')
    } catch (requestError) {
      if (
        requestError instanceof ApiClientError &&
        requestError.code === 'password_mismatch'
      ) {
        setErrorMessage('The two new passwords do not match.')
      } else if (
        requestError instanceof ApiClientError &&
        requestError.code === 'invalid_current_password'
      ) {
        setErrorMessage('Current password is invalid.')
      } else {
        setErrorMessage(
          requestError instanceof Error
            ? requestError.message
            : 'Unable to update password.',
        )
      }
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <section className="page-card">
      <p className="panel-title">Security</p>
      <h2>Change Password</h2>

      <form className="stack-form" onSubmit={onSubmit}>
        <label>
          Current password
          <input
            type="password"
            value={currentPassword}
            onChange={(event) => {
              setCurrentPassword(event.target.value)
            }}
            autoComplete="current-password"
            required
          />
        </label>

        <label>
          New password
          <input
            type="password"
            value={newPassword}
            onChange={(event) => {
              setNewPassword(event.target.value)
            }}
            autoComplete="new-password"
            required
          />
        </label>

        <label>
          Confirm new password
          <input
            type="password"
            value={newPasswordCheck}
            onChange={(event) => {
              setNewPasswordCheck(event.target.value)
            }}
            autoComplete="new-password"
            required
          />
        </label>

        <button className="primary-button" type="submit" disabled={isSubmitting}>
          {isSubmitting ? 'Updating...' : 'Update password'}
        </button>
      </form>

      {successMessage ? <p className="status-note status-note-ok">{successMessage}</p> : null}
      {errorMessage ? <p className="status-note status-note-error">{errorMessage}</p> : null}
    </section>
  )
}
