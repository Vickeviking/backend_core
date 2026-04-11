import { useCallback, useEffect, useState, type ReactNode } from 'react'
import { NavLink, Navigate, Outlet, Route, Routes, useNavigate } from 'react-router-dom'
import './App.css'
import {
  apiBaseUrl,
  getSession,
  logout,
  type AuthSessionResponse,
} from './lib/api'
import { AdminDashboardPage } from './pages/admin-dashboard-page'
import { AdminNewslettersPage } from './pages/admin-newsletters-page'
import { AdminPasswordPage } from './pages/admin-password-page'
import { LoginPage } from './pages/login-page'
import { NotFoundPage } from './pages/not-found-page'
import { PublicSubscriptionPage } from './pages/public-subscription-page'
import { SubscriptionConfirmPage } from './pages/subscription-confirm-page'

const adminNavigation = [
  {
    to: '/admin/dashboard',
    label: 'Dashboard',
  },
  {
    to: '/admin/password',
    label: 'Password',
  },
  {
    to: '/admin/newsletters',
    label: 'Newsletters',
  },
]

function App() {
  const navigate = useNavigate()
  const [session, setSession] = useState<AuthSessionResponse>({
    authenticated: false,
  })
  const [isBootstrapping, setIsBootstrapping] = useState(true)
  const [bootstrapError, setBootstrapError] = useState<string | null>(null)

  const refreshSession = useCallback(async (signal?: AbortSignal) => {
    const nextSession = await getSession(signal)
    setSession(nextSession)
  }, [])

  useEffect(() => {
    const controller = new AbortController()

    void (async () => {
      try {
        await refreshSession(controller.signal)
      } catch (error) {
        if (controller.signal.aborted) {
          return
        }

        const message =
          error instanceof Error
            ? error.message
            : 'Failed to bootstrap session state.'
        setBootstrapError(message)
      } finally {
        setIsBootstrapping(false)
      }
    })()

    return () => {
      controller.abort()
    }
  }, [refreshSession])

  async function handleLogout() {
    try {
      await logout()
      await refreshSession()
      navigate('/login', { replace: true })
    } catch (error) {
      setBootstrapError(
        error instanceof Error ? error.message : 'Failed to terminate session.',
      )
    }
  }

  if (isBootstrapping) {
    return (
      <main className="boot-screen">
        <p>Bootstrapping admin session...</p>
      </main>
    )
  }

  return (
    <div className="app-root">
      <div className="bg-halo bg-halo-top" aria-hidden="true" />
      <div className="bg-halo bg-halo-bottom" aria-hidden="true" />

      <header className="topbar">
        <div>
          <p className="topbar-eyebrow">Backend Core</p>
          <h1>Admin + Subscription Console</h1>
        </div>
        <div className="topbar-meta">
          <span>API</span>
          <code>{apiBaseUrl}</code>
          {session.authenticated && session.username ? (
            <p className="session-pill">Signed in as {session.username}</p>
          ) : (
            <p className="session-pill">Not signed in</p>
          )}
        </div>
      </header>

      {bootstrapError ? <p className="global-error">{bootstrapError}</p> : null}

      <div className="page-shell">
        <Routes>
          <Route path="/" element={<PublicSubscriptionPage />} />
          <Route path="/login" element={<LoginRouteGuard authenticated={session.authenticated}><LoginPage onLoginSuccess={refreshSession} /></LoginRouteGuard>} />
          <Route path="/subscriptions/confirm" element={<SubscriptionConfirmPage />} />

          <Route element={<RequireAuthenticated authenticated={session.authenticated} />}>
            <Route
              element={<AdminLayout onLogout={handleLogout} />}
            >
              <Route
                path="/admin/dashboard"
                element={<AdminDashboardPage username={session.username} />}
              />
              <Route path="/admin/password" element={<AdminPasswordPage />} />
              <Route path="/admin/newsletters" element={<AdminNewslettersPage />} />
            </Route>
          </Route>

          <Route path="*" element={<NotFoundPage />} />
        </Routes>
      </div>
    </div>
  )
}

function LoginRouteGuard({
  authenticated,
  children,
}: {
  authenticated: boolean
  children: ReactNode
}) {
  if (authenticated) {
    return <Navigate to="/admin/dashboard" replace />
  }
  return <>{children}</>
}

function RequireAuthenticated({ authenticated }: { authenticated: boolean }) {
  if (!authenticated) {
    return <Navigate to="/login" replace />
  }
  return <Outlet />
}

function AdminLayout({ onLogout }: { onLogout: () => Promise<void> }) {
  const [isLoggingOut, setIsLoggingOut] = useState(false)

  return (
    <section className="admin-shell">
      <aside className="admin-sidebar">
        <p className="panel-title">Admin</p>
        <nav className="admin-nav" aria-label="Admin navigation">
          {adminNavigation.map((entry) => (
            <NavLink
              key={entry.to}
              to={entry.to}
              className={({ isActive }) =>
                isActive ? 'admin-nav-link admin-nav-link-active' : 'admin-nav-link'
              }
            >
              {entry.label}
            </NavLink>
          ))}
        </nav>

        <button
          type="button"
          className="ghost-button"
          disabled={isLoggingOut}
          onClick={() => {
            void (async () => {
              setIsLoggingOut(true)
              try {
                await onLogout()
              } finally {
                setIsLoggingOut(false)
              }
            })()
          }}
        >
          {isLoggingOut ? 'Signing out...' : 'Sign out'}
        </button>
      </aside>

      <main className="admin-content">
        <Outlet />
      </main>
    </section>
  )
}

export default App
