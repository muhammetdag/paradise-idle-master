import { writable } from 'svelte/store';

export interface ModalSettings {
  show: boolean;
  title: string;
  message: string;
  alertOnly?: boolean;
  onConfirm?: () => void | Promise<void>;
  onCancel?: () => void;
}

const initial: ModalSettings = {
  show: false,
  title: '',
  message: '',
  alertOnly: false,
};

function createModalStore() {
  const { subscribe, set, update } = writable<ModalSettings>(initial);

  return {
    subscribe,
    confirm: (settings: Omit<ModalSettings, 'show'>) => {
      set({ ...settings, show: true, alertOnly: false });
    },
    alert: (title: string, message: string) => {
      set({ 
        show: true, 
        title, 
        message, 
        alertOnly: true, 
        onConfirm: () => update(s => ({ ...s, show: false })) 
      });
    },
    close: () => {
      update(s => {
        if (s.onCancel) s.onCancel();
        return { ...s, show: false };
      });
    },
    handleConfirm: async () => {
      let callback: (() => void | Promise<void>) | undefined;
      update(s => {
        callback = s.onConfirm;
        return { ...s, show: false };
      });
      if (callback) await callback();
    }
  };
}

export const modal = createModalStore();
