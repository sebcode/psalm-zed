# psalm Zed Extension

[Psalm](https://psalm.dev/) is a static analysis tool for PHP that helps detect
type errors, bugs, and code quality issues without running the code. It
improves code reliability and maintainability by enforcing type safety and
identifying potential problems early in development.

## Configuration

By default, the psalm.xml file is required to be in the root of the workspace.

Otherwise, it can be configured through the lsp settings:

```
{
  "lsp": {
    "psalm": {
      "settings": {
        "config_path": "path/to/psalm.xml"
      }
    }
  }
}
```

### Specify binary

By default, this extension will pick up the `psalm` binary available in the current `$PATH`.
It can also be configured like this:

```
{
  "lsp": {
    "psalm": {
      "settings": {
        "binary": "vendor/bin/psalm"
      }
    }
  }
}
```

### Enable psalm only when psalm.xml is present

Enable `require_config_file` to only run psalm if `psalm.xml` is present.

```
{
  "lsp": {
    "psalm": {
      "settings": {
        "require_config_file": true
      }
    }
  }
}
```
