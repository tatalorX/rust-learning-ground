import { describe, it, expect, beforeEach } from "vitest";
import "../../../src/components/atoms/Button";

describe("RLButton", () => {
  beforeEach(() => {
    document.body.innerHTML = "";
  });

  it("renders with default variant", () => {
    document.body.innerHTML = "<rl-button>Click me</rl-button>";
    const button = document.querySelector("rl-button");
    expect(button).toBeTruthy();
    expect(button?.textContent).toBe("Click me");
  });

  it("renders with primary variant by default", () => {
    document.body.innerHTML = "<rl-button>Primary Button</rl-button>";
    const button = document.querySelector("rl-button");
    const shadowButton = button?.shadowRoot?.querySelector("button");
    expect(shadowButton?.classList.contains("btn-primary")).toBe(true);
  });

  it("renders with secondary variant", () => {
    document.body.innerHTML =
      '<rl-button variant="secondary">Secondary Button</rl-button>';
    const button = document.querySelector("rl-button");
    const shadowButton = button?.shadowRoot?.querySelector("button");
    expect(shadowButton?.classList.contains("btn-secondary")).toBe(true);
  });

  it("renders with success variant", () => {
    document.body.innerHTML =
      '<rl-button variant="success">Success Button</rl-button>';
    const button = document.querySelector("rl-button");
    const shadowButton = button?.shadowRoot?.querySelector("button");
    expect(shadowButton?.classList.contains("btn-success")).toBe(true);
  });

  it("renders with ghost variant", () => {
    document.body.innerHTML =
      '<rl-button variant="ghost">Ghost Button</rl-button>';
    const button = document.querySelector("rl-button");
    const shadowButton = button?.shadowRoot?.querySelector("button");
    expect(shadowButton?.classList.contains("btn-ghost")).toBe(true);
  });

  it("renders with different sizes", () => {
    const sizes = ["sm", "md", "lg"];
    sizes.forEach((size) => {
      document.body.innerHTML = `<rl-button size="${size}">${size} Button</rl-button>`;
      const button = document.querySelector("rl-button");
      const shadowButton = button?.shadowRoot?.querySelector("button");
      expect(shadowButton?.classList.contains(`size-${size}`)).toBe(true);
    });
  });

  it("emits click event when clicked", () => {
    document.body.innerHTML = "<rl-button>Click me</rl-button>";
    const button = document.querySelector("rl-button");

    let clicked = false;
    button?.addEventListener("click", () => {
      clicked = true;
    });

    const shadowButton = button?.shadowRoot?.querySelector("button");
    shadowButton?.click();
    expect(clicked).toBe(true);
  });

  it("does not emit click when disabled", () => {
    document.body.innerHTML = "<rl-button disabled>Click me</rl-button>";
    const button = document.querySelector("rl-button");

    let clicked = false;
    button?.addEventListener("click", () => {
      clicked = true;
    });

    const shadowButton = button?.shadowRoot?.querySelector("button");
    shadowButton?.click();
    expect(clicked).toBe(false);
    expect(shadowButton?.disabled).toBe(true);
  });

  it("does not emit click when loading", () => {
    document.body.innerHTML = "<rl-button loading>Loading...</rl-button>";
    const button = document.querySelector("rl-button");

    let clicked = false;
    button?.addEventListener("click", () => {
      clicked = true;
    });

    const shadowButton = button?.shadowRoot?.querySelector("button");
    shadowButton?.click();
    expect(clicked).toBe(false);
    expect(shadowButton?.disabled).toBe(true);
  });

  it("shows spinner when loading", () => {
    document.body.innerHTML = "<rl-button loading>Loading...</rl-button>";
    const button = document.querySelector("rl-button");
    const spinner = button?.shadowRoot?.querySelector(".spinner");
    expect(spinner).toBeTruthy();
  });

  it("does not show spinner when not loading", () => {
    document.body.innerHTML = "<rl-button>Not Loading</rl-button>";
    const button = document.querySelector("rl-button");
    const spinner = button?.shadowRoot?.querySelector(".spinner");
    expect(spinner).toBeFalsy();
  });

  it("supports full-width attribute", () => {
    document.body.innerHTML = "<rl-button full-width>Full Width</rl-button>";
    const button = document.querySelector("rl-button");
    const shadowButton = button?.shadowRoot?.querySelector("button");
    expect(shadowButton?.style.width).toBe("100%");
  });

  it("supports different button types", () => {
    const types = ["button", "submit", "reset"];
    types.forEach((type) => {
      document.body.innerHTML = `<rl-button type="${type}">${type} Button</rl-button>`;
      const button = document.querySelector("rl-button");
      const shadowButton = button?.shadowRoot?.querySelector("button");
      expect(shadowButton?.getAttribute("type")).toBe(type);
    });
  });

  it("renders slot content correctly", () => {
    document.body.innerHTML = "<rl-button>Custom Label</rl-button>";
    const button = document.querySelector("rl-button");
    expect(button?.textContent).toBe("Custom Label");
  });
});
