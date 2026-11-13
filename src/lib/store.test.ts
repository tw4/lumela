import { beforeEach, describe, expect, it } from "vitest";
import { DEFAULT_SETTINGS } from "./beautify";
import { useStore } from "./store";

describe("presets", () => {
  beforeEach(() => useStore.setState({ beautify: DEFAULT_SETTINGS, presets: [], activePresetId: null, lastPresetId: null }));

  it("saves, applies and remembers the last preset", () => {
    const s = useStore.getState();
    s.setBeautify({ padding: 40 });
    const id = useStore.getState().savePreset("Blog");
    useStore.getState().setBeautify({ padding: 5 });
    expect(useStore.getState().activePresetId).toBeNull();
    useStore.getState().applyPreset(id);
    expect(useStore.getState().beautify.padding).toBe(40);
    expect(useStore.getState().activePresetId).toBe(id);
    expect(useStore.getState().lastPresetId).toBe(id);
  });

  it("overwrites a preset with the same name and deletes", () => {
    useStore.getState().savePreset("A");
    useStore.getState().setBeautify({ radius: 30 });
    const id = useStore.getState().savePreset("A");
    expect(useStore.getState().presets).toHaveLength(1);
    expect(useStore.getState().presets[0].beautify.radius).toBe(30);
    useStore.getState().deletePreset(id);
    expect(useStore.getState().presets).toHaveLength(0);
    expect(useStore.getState().lastPresetId).toBeNull();
  });

  it("saves 'No background' in a preset", () => {
    useStore.getState().setBeautify({ frame: false });
    const id = useStore.getState().savePreset("Plain");
    useStore.getState().setBeautify({ frame: true });
    useStore.getState().applyPreset(id);
    expect(useStore.getState().beautify.frame).toBe(false);
  });

  it("keeps the frame on for presets saved before 'No background' existed", () => {
    const { frame: _, ...old } = DEFAULT_SETTINGS;
    useStore.setState({ presets: [{ id: "old", name: "Old", beautify: old as typeof DEFAULT_SETTINGS }] });
    useStore.getState().applyPreset("old");
    expect(useStore.getState().beautify.frame).toBe(true);
  });
});
