# `pipfile-diff`
A simple tool to compare changes in `Pipfile.lock`-files. By default, it compares your `Pipfile.lock` to the latest version committed to git.

## Usage

To compare a freshly locked `Pipfile.lock` to the latest commited one, run

```
$ pipfile-diff path/to/Pipfile.lock 

Default:
Changed:
  certifi: 2022.5.18.1 => 2022.9.24
  lxml: 4.9.0 => 4.9.1
  tinycss2: 1.1.1 => 1.2.1
  pytz: 2022.1 => 2022.5
  django-filer: 2.2.1 => 2.2.3
  sentry-sdk: 1.5.1 => 1.10.1
  djangocms-picture: 3.0.0 => 4.0.0
  django-mptt: 0.13.4 => 0.14.0
  django-formtools: 2.3 => 2.4
  sqlparse: 0.4.2 => 0.4.3
  easy-thumbnails: 2.8 => 2.8.3
  django: 3.2.13 => 3.2.16
  django-polymorphic: 3.0.0 => 3.1.0
  djangocms-link: 3.0.0 => 3.1.0
  reportlab: 3.6.9 => 3.6.12
  django-analytical: 3.0.0 => 3.1.0
  svglib: 1.3.0 => 1.4.1
  djangocms-admin-style: 3.1.1 => 3.2.0
  djangocms-text-ckeditor: 4.0.0 => 5.1.1
  pillow: 9.1.1 => 9.2.0
  django-haystack: 3.1.1 => 3.2.1
  django-treebeard: 4.5.1 => 4.4
  django-widget-tweaks: 1.4.9 => 1.4.12
  urllib3: 1.26.9 => 1.26.12
  django-sekizai: 2.0.0 => 4.0.0
  django-recaptcha: 2.0.6 => 3.0.0
  djangocms-attributes-field: 2.0.0 => 2.1.0
  cssselect2: 0.6.0 => 0.7.0
  setuptools: 62.3.2 => 65.5.0

Development:
Changed:
  django: 3.2.13 => 3.2.16
  dj-database-url: 0.5.0 => 1.0.0
  astroid: 2.9.3 => 2.12.12
  tzdata: 2022.1 => 2022.5
  mccabe: 0.6.1 => 0.7.0
  pylint-django: 2.4.4 => 2.5.3
  coverage: 6.2 => 6.5.0
  sqlparse: 0.4.2 => 0.4.3
  wrapt: 1.13.3 => 1.14.1
  lazy-object-proxy: 1.7.1 => 1.8.0
  django-debug-toolbar: 3.2.3 => 3.7.0
  pylint: 2.12.2 => 2.15.5
New:
  tomlkit: 0.11.6
  pytz: 2022.5
  tomli: 2.0.1
  dill: 0.3.6
Deleted:
  setuptools: 62.3.2
  toml: 0.10.2
```

## Installation

### From source

To install `pipfile-diff` from source, clone the repository and run

```
cargo install --path .
```

## License

`pipfile-diff` is licensed under the Apache-2.0 license.
