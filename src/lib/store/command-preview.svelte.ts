export type OpenCommand = {
  url: string;
  screen: string;
  user: string;
  flag?: string;
  date?: string;
}

export type TestCommand = {
  scenarioName?: string;
  featureFile: string;
}

export type EmptyCommand = {
  isEmptyCommand: true;
}

export type CommandCollection = OpenCommand | TestCommand | EmptyCommand;

class CommandPreview {
  command = $state('command');
}

export default new CommandPreview();
