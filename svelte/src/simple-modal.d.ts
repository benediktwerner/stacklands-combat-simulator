import type {
  ComponentType,
  SvelteComponent,
  SvelteComponentTyped,
} from 'svelte';
import type { BlurParams, TransitionConfig } from 'svelte/transition';

export interface CloseCallbacks {
  onClose: () => void;
  onClosed: () => void;
}

export interface OpenCallbacks extends CloseCallbacks {
  onOpen: () => void;
  onOpened: () => void;
}

export interface Options<
  BgTransParams = BlurParams,
  WinTransParams = BlurParams
> {
  ariaLabel?: string | null;
  ariaLabelledBy?: string | null;
  /** Default: true */
  closeButton?: SvelteComponent | boolean;
  /** Default: true */
  closeOnEsc?: boolean;
  /** Default: true */
  closeOnOuterClick?: boolean;
  styleBg?: Record<string, string | number>;
  styleWindowWrap?: Record<string, string | number>;
  styleWindow?: Record<string, string | number>;
  styleContent?: Record<string, string | number>;
  styleCloseButton?: Record<string, string | number>;
  classBg?: string | null;
  classWindowWrap?: string | null;
  classWindow?: string | null;
  classContent?: string | null;
  classCloseButton?: string | null;
  /**
   * Default: fade
   * @see https://svelte.dev/docs#run-time-svelte-transition
   * */
  transitionBg?: (node: Element, parameters: BgTransParams) => TransitionConfig;
  transitionBgProps?: BgTransParams;
  /**
   * Default: fade
   * @see https://svelte.dev/docs#run-time-svelte-transition
   * */
  transitionWindow?: (
    node: Element,
    parameters: BgTransParams
  ) => TransitionConfig;
  transitionWindowProps?: BgTransParams;
  /** Default: false */
  disableFocusTrap?: boolean;
  /**
   * A function to determine if an HTML element is tabbable
   */
  isTabbable?: (node: Element) => boolean;
  /** Default: false */
  unstyled?: boolean;
}

export interface SimpleModalContext {
  open: <
    C extends SvelteComponentTyped,
    BgTransParams = BlurParams,
    WinTransParams = BlurParams
  >(
    NewComponent: ComponentType<C>,
    newProps?: C extends SvelteComponentTyped<infer Props> ? Props : never,
    options?: Options<BgTransParams, WinTransParams>,
    callback?: OpenCallbacks
  ) => void;
  close: (callback?: CloseCallbacks) => void;
}
