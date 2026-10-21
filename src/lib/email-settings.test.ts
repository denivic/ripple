import { describe, expect, it } from "vitest";

import { dtoFromForm, formFromDto, providerHint, secretLabel, type EmailForm } from "./email-settings";

const form: EmailForm = {
  provider: "smtp",
  fromAddress: " me@example.com ",
  toAddress: "me@example.com",
  smtpHost: "smtp.gmail.com",
  smtpPort: "587",
  smtpUsername: "",
  smtpSecurity: "starttls",
};

describe("email settings form", () => {
  it("round-trips through the DTO, trimming and nulling blanks", () => {
    const result = dtoFromForm(form);
    expect("dto" in result).toBe(true);
    if (!("dto" in result)) return;
    expect(result.dto.fromAddress).toBe("me@example.com");
    expect(result.dto.smtpUsername).toBeNull();
    expect(result.dto.smtpPort).toBe(587);
    expect(formFromDto(result.dto).smtpPort).toBe("587");
  });

  it("rejects out-of-range or non-integer ports and allows a blank one", () => {
    for (const smtpPort of ["0", "70000", "58.7", "abc"]) {
      expect(dtoFromForm({ ...form, smtpPort })).toEqual({ error: "SMTP port must be between 1 and 65535" });
    }
    const blank = dtoFromForm({ ...form, smtpPort: " " });
    expect("dto" in blank && blank.dto.smtpPort).toBeNull();
  });

  it("labels the secret per provider", () => {
    expect(secretLabel("smtp")).toBe("Password");
    expect(secretLabel("resend")).toBe("Resend API key");
    expect(secretLabel("postmark")).toBe("Postmark server token");
  });

  it("gives Gmail and Proton specific hints", () => {
    expect(providerHint({ provider: "smtp", smtpHost: "smtp.gmail.com" })).toMatch(/app password/);
    expect(providerHint({ provider: "smtp", smtpHost: "127.0.0.1" })).toMatch(/Bridge/);
    expect(providerHint({ provider: "none", smtpHost: "" })).toBeNull();
  });
});
