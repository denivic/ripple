// Pure helpers behind EmailSettings.svelte, kept out of the component so the
// form logic is testable without rendering.

import type { EmailProviderDto, EmailSettingsDto, SmtpSecurityDto } from "./ipc";

/** The form's editable state; every field is a string as typed. */
export interface EmailForm {
  provider: EmailProviderDto;
  fromAddress: string;
  toAddress: string;
  smtpHost: string;
  smtpPort: string;
  smtpUsername: string;
  smtpSecurity: SmtpSecurityDto;
}

export const PROVIDER_OPTIONS: { value: EmailProviderDto; label: string }[] = [
  { value: "none", label: "None" },
  { value: "smtp", label: "SMTP" },
  { value: "resend", label: "Resend" },
  { value: "postmark", label: "Postmark" },
];

export const SECURITY_OPTIONS: { value: SmtpSecurityDto; label: string }[] = [
  { value: "starttls", label: "STARTTLS (usually port 587)" },
  { value: "tls", label: "TLS (usually port 465)" },
];

/** What the provider calls its secret — the field label. */
export function secretLabel(provider: EmailProviderDto): string {
  switch (provider) {
    case "smtp":
      return "Password";
    case "resend":
      return "Resend API key";
    case "postmark":
      return "Postmark server token";
    default:
      return "Password";
  }
}

/** Provider-specific setup advice, keyed on what the user has typed so far. */
export function providerHint(form: Pick<EmailForm, "provider" | "smtpHost">): string | null {
  if (form.provider === "smtp") {
    const host = form.smtpHost.trim().toLowerCase();
    if (host.includes("gmail") || host.includes("google")) {
      return "Gmail needs an app password (Google Account → Security → App passwords), not your normal password.";
    }
    if (host.includes("proton") || host === "127.0.0.1" || host === "localhost") {
      return "Proton Mail only sends over SMTP through Proton Mail Bridge; use the host, port and password Bridge shows you.";
    }
    return "For Gmail use an app password. Proton Mail requires Proton Mail Bridge.";
  }
  if (form.provider === "resend") {
    return "The From address must be on a domain you have verified in Resend.";
  }
  if (form.provider === "postmark") {
    return "Use a Server API token. The From address must be a confirmed Postmark sender signature.";
  }
  return null;
}

export function formFromDto(dto: EmailSettingsDto): EmailForm {
  return {
    provider: dto.provider,
    fromAddress: dto.fromAddress ?? "",
    toAddress: dto.toAddress ?? "",
    smtpHost: dto.smtpHost ?? "",
    smtpPort: dto.smtpPort == null ? "" : String(dto.smtpPort),
    smtpUsername: dto.smtpUsername ?? "",
    smtpSecurity: dto.smtpSecurity,
  };
}

const blankToNull = (s: string): string | null => (s.trim() === "" ? null : s.trim());

/**
 * Builds the DTO to save, or a readable error for the port — the one field
 * the backend can't receive as typed. Addresses are validated server-side.
 */
export function dtoFromForm(form: EmailForm): { dto: EmailSettingsDto } | { error: string } {
  let smtpPort: number | null = null;
  const port = form.smtpPort.trim();
  if (port !== "") {
    const n = Number(port);
    if (!Number.isInteger(n) || n < 1 || n > 65535) {
      return { error: "SMTP port must be between 1 and 65535" };
    }
    smtpPort = n;
  }
  return {
    dto: {
      provider: form.provider,
      fromAddress: blankToNull(form.fromAddress),
      toAddress: blankToNull(form.toAddress),
      smtpHost: blankToNull(form.smtpHost),
      smtpPort,
      smtpUsername: blankToNull(form.smtpUsername),
      smtpSecurity: form.smtpSecurity,
      hasSecret: false,
    },
  };
}
