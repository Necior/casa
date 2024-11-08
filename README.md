# casa

A simple (SQLite-based, no authentication, no JavaScript) PWA (Progressive Web App) to track household expenses.

## Usage

Set up the SQLite database in `db.sqlite` and then run:

```sh
nix run
```

If you're one of today's lucky 10000 and don't know what Nix is, check out [https://nixos.org/](https://nixos.org/).

## Background

I wanted an app that would:

* be dedicated for my household needs;
* require minimal effort to enter a new expense;
* provide value;
* be designed in line with _my_ threat model;
* be fun or educational to develop.

That resulted in the following implementation decisions:

* Progressive Web App so it looks like an app on a phone and it might be used from a browser;
* The most common workflow -- entering a new expense -- is the first thing on the main page;
* SQLite as the database since we don't need high availability;
* No authentication in the app layer. When deploying, use whatever other mechanism to limit access. It might be a reverse proxy, or an air-gapped machine in your hall :)
* Manual database migrations;
* No JavaScript;
* [Water.css](https://watercss.kognise.dev/) as the collection of CSS styles;
* Coded in Rust.
