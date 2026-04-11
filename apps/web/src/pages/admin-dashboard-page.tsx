import { Link } from 'react-router-dom'

export function AdminDashboardPage({ username }: { username?: string }) {
  return (
    <section className="page-card">
      <p className="panel-title">Dashboard</p>
      <h2>Welcome {username ?? 'admin'}</h2>
      <p>
        This panel is now fully frontend-rendered. Backend responsibilities stay focused on
        auth, validation, and business logic under /api.
      </p>

      <div className="feature-grid">
        <article className="feature-card">
          <h3>Publish Issue</h3>
          <p>Create and enqueue a newsletter issue for confirmed subscribers.</p>
          <Link className="text-link" to="/admin/newsletters">
            Open newsletters
          </Link>
        </article>

        <article className="feature-card">
          <h3>Update Credentials</h3>
          <p>Rotate your admin password using the secure API workflow.</p>
          <Link className="text-link" to="/admin/password">
            Change password
          </Link>
        </article>
      </div>
    </section>
  )
}
