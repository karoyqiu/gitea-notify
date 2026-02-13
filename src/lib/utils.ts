import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export type ClassNamesType = {
  root?: ClassValue;
  trigger?: ClassValue;
  content?: ClassValue;
  item?: ClassValue;
};

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
