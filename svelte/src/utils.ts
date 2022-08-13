export const sleep = (): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve));
};
