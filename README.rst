OhSh*t
======

**Your program fucked up. Here's why.**

.. warning::

	OhSh*t is under active development and is not ready for use in projects.
	The API, behavior, and build workflow may change without notice. There is
	no supported release yet.

OhSh*t is a Rust logging and diagnostics library for the moments when a
program fails in a way that needs clear, honest explanation. It tries to
answer the questions that matter most when a failure happens:

* What happened
* Where it happened
* Why it happened when the cause is known
* What the failure means
* What the user can do next when a useful action is known

The goal is not to hide uncertainty. The goal is to be specific, readable,
and useful without pretending a cause or fix exists when the evidence does not
support it.

Why it exists
-------------

Most logging systems are fine for routine status output. They are often less
helpful when a process is in trouble. Generic output like "ERROR: operation
failed" is not enough when a human needs to fix the problem.

OhSh*t is built for that gap: a library that produces brutal honesty, useful
context, and actionable diagnostics instead of vague failure noise.

Availability
------------

OhSh*t has not been published to crates.io and should not be added as a
dependency yet. The current ``0.1.0`` version is development metadata, not a
stable release.

For local experimentation only, clone the repository and use the development
workflow described below. Local builds are unsupported and should not be used
as a production dependency.

.. code-block:: console

	$ git clone https://github.com/playfairs/ohshit
	$ cd ohshit
	$ nix develop

API preview
-----------

The following examples show the current direction of the API. They are
unstable and may stop compiling as the project develops.

Basic usage
~~~~~~~~~~~

.. code-block:: rust

	use ohshit::info;

	fn main() {
		 ohshit::init();

		 info!("server started");
		 info!(port = 8000; "listening on port {}", 8000);
	}

Structured logging
~~~~~~~~~~~~~~~~~~

The library supports contextual fields without forcing every log line into a
complicated abstraction.

.. code-block:: rust

	use ohshit::warn;

	warn!(
		 host = "127.0.0.1",
		 port = 5432;
		 "database connection failed"
	);

Diagnostics
~~~~~~~~~~~

The differentiating feature is the diagnostic model: a failure can carry a
headline, a cause, a reason, context, and a suggested action.

.. code-block:: rust

	use ohshit::Diagnostic;

	let diagnostic = Diagnostic::new("Could not bind to 0.0.0.0:8080")
		 .with_cause("Address already in use")
		 .with_reason("Another process is already listening on port 8080.")
		 .with_action("lsof -i :8080");

	println!("{}", diagnostic.render());

This produces output in the style of::

	OH SHIT

	Could not bind to 0.0.0.0:8080

	Reason:
	  Another process is already listening on port 8080.

	Cause:
	  Address already in use

	Try:
	  lsof -i :8080

When evidence is weak, the library does not invent a cause. It makes
uncertainty explicit instead of guessing.

Development status
------------------

This project is experimental and incomplete. It is being developed as a
library first, with examples used to exercise the API. Compatibility,
documentation, packaging, and production-readiness work are still pending.

Development workflow
--------------------

The project is designed around a simple model:

* log records are data
* log levels describe the seriousness of the event
* logger configuration controls output behavior
* formatting stays separate from event creation
* sinks and filters can evolve without rewriting the core API

This keeps the initial version small and honest while leaving space for
expansion. Contributions and local testing are welcome, but users should
expect breaking changes.

Building with Nox
-----------------

The primary build interface is Nox.

.. code-block:: console

	$ nox build
	$ nox clean

Nix development environment
---------------------------

The repository includes a Nix flake for local development.

.. code-block:: console

	$ nix develop
	$ nix fmt
	$ nix flake check

This brings in the Nox tool from the pinned flake input, plus the Rust
toolchain and formatting utilities needed for development and validation.