import { Link } from 'react-router-dom'

export function NotFoundPage() {
  return (
    <section className="page-card">
      <p className="panel-title">Missing route</p>
      <h2>This page does not exist</h2>

      <p>
        Use one of the known entry points to continue.
      </p>

      <p>
        <Link className="text-link" to="/">
          Back to subscription page
        </Link>
      </p>
    </section>
  )
}
