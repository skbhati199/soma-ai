Name:           soma-ai
Version:        0.1.0
Release:        1%{?dist}
Summary:        Local AI terminal command assistant

License:        MIT
URL:            https://github.com/skbhati199/soma-ai
Source0:        https://github.com/skbhati199/soma-ai/releases/download/v%{version}/soma-x86_64-unknown-linux-gnu.tar.gz

BuildArch:      x86_64
BuildRequires:  tar gzip

%description
Soma AI is a single-binary, local AI command assistant for your terminal.
Works offline after first model download. Supports PowerShell, Bash, Zsh, and Fish.

%prep
%autosetup -n soma

%build
# No build needed, binary is pre-built

%install
mkdir -p %{buildroot}%{_bindir}
install -m 755 soma %{buildroot}%{_bindir}/soma

%files
%{_bindir}/soma
%doc README.md LICENSE

%changelog
* Wed Oct 25 2024 skbhati199 <skbhati199@gmail.com> - 0.1.0-1
- Initial package
