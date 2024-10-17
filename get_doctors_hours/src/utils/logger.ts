import { blue, red, yellow, green } from "chalk";

export class Logger {
  private static instance: Logger;

  private constructor() {}

  public static getInstance() {
    if (!Logger.instance) {
      Logger.instance = new Logger();
    }
    return Logger.instance;
  }

  public log(message: string) {
    console.log(blue(message));
  }

  public error(message: string) {
    console.error(red(message));
  }

  public warn(message: string) {
    console.warn(yellow(message));
  }

  public success(message: string) {
    console.log(green(message));
  }
}
