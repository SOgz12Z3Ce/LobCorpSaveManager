using System;
using System.Collections.Generic;
using System.IO;
using System.Security.Permissions;
using LobCorp.Exceptions;
using LobCorp.Save;
using LobCorp.Save.Serializers;
using Newtonsoft.Json;
using Newtonsoft.Json.Schema;

namespace LobCorp
{
	internal class Program
	{
		static int Main(string[] args)
		{
			if (args.Length == 0)
			{
				Console.WriteLine("Usage: lobcss [command] [options] file");
				Console.WriteLine("Commands:");
				Console.WriteLine("  decode  Decode binary save file to JSON.");
				Console.WriteLine("  encode  Encode JSON save file to bianry file.");
				Console.WriteLine("Options:");
				Console.WriteLine("  -f --format  Output in formatted form.");
				return 1;
			}
			Parameter parameter;
			try
			{
				parameter = new Parameter(args);
			}
			catch (BadArguments e)
			{
				Console.WriteLine(e.Message);
				return 1;
			}

			if (parameter.Cmd == null)
			{
				Fail("No command specified.");
			}
			switch (parameter.Cmd)
			{
				case Parameter.Command.Decode:
					{
						if (parameter.FilePath == null)
						{
							Fail("No file specified.");
						}
						SaveFile save = null;
						try
						{
							save = BinarySaveSerializer.Deserialize(parameter.FilePath);
						}
						catch (FileNotFoundException)
						{
							Fail(string.Format("File dosen't exist: {0}", parameter.FilePath));
						}
						catch (Exception)
						{
							Fail(string.Format("Unable to deserialze file: {0}", parameter.FilePath));
						}

						if (parameter.HasOption(Parameter.Option.SaveToFile))
						{
							if (File.Exists(save.DefaultJsonFileName))
							{
								Fail(string.Format("Output file {0} already exist.", save.DefaultJsonFileName));
							}
							if (parameter.HasOption(Parameter.Option.Format))
							{
								JsonSaveSerializer.Serialize(save.DefaultJsonFileName, save, Formatting.Indented);
							}
							else
							{
								JsonSaveSerializer.Serialize(save.DefaultJsonFileName, save);
							}
						}
						else
						{
							if (parameter.HasOption(Parameter.Option.Format))
							{
								Console.WriteLine(save.ToJson(Formatting.Indented));
							}
							else
							{
								Console.WriteLine(save.ToJson());
							}
						}
						break;
					}
				case Parameter.Command.Encode:
					{
						if (parameter.FilePath == null)
						{
							Fail("No file specified.");
						}
						SaveFile save = null;
						try
						{
							save = JsonSaveSerializer.Deserialize(parameter.FilePath);
						}
						catch (FileNotFoundException)
						{
							Fail(string.Format("File dosen't exist: {0}", parameter.FilePath));
						}
						catch (Exception)
						{
							Fail(string.Format("Unable to deserialze file: {0}", parameter.FilePath));
						}
						if (File.Exists(save.DefaultBinaryFileName))
						{
							Fail(string.Format("Output file {0} already exist.", save.DefaultBinaryFileName));
						}

						BinarySaveSerializer.Serialize(save.DefaultBinaryFileName, save);
						break;
					}
			}
			return 0;
		}
		static private void Fail(string message)
		{
			Console.WriteLine(message);
			Console.WriteLine("Run lobcss for help info.");
			Environment.Exit(1);
		}
		private class Parameter
		{
			public readonly string FilePath;
			public readonly Command? Cmd;
			private static readonly Dictionary<string, Option> optionMap = new Dictionary<string, Option>()
			{
				{ "-f", Option.Format},
				{ "--format", Option.Format},
				{ "-s", Option.SaveToFile},
				{ "--save-to-file", Option.SaveToFile},
			};
			private static readonly Dictionary<string, Command> commandMap = new Dictionary<string, Command>()
			{
				{ "decode", Command.Decode},
				{ "encode", Command.Encode},
			};
			private readonly Dictionary<Option, bool> options = new Dictionary<Option, bool>();
			public Parameter(string[] args)
			{
				for (int i = 0; i < args.Length; i++)
				{
					string cur = args[i];

					// options
					if (cur.StartsWith("-"))
					{
						Option? option = OptionOf(cur);
						if (option == null)
						{
							throw new BadArguments(string.Format("Unable to resolve option: {0}", cur));
						}
						if (options.ContainsKey(option.Value))
						{
							throw new BadArguments(string.Format("Repeating option: {0}", cur));
						}
						options[option.Value] = true;
						continue;
					}

					// command
					if (Cmd == null)
					{
						Command? command = CommandOf(cur);
						if (command == null)
						{
							throw new BadArguments(string.Format("Unable to resolve command: {0}", cur));
						}
						Cmd = command;
						continue;
					}

					// filepath
					if (FilePath != null)
					{
						throw new BadArguments(string.Format("Too many arguments: {0}", cur));
					}
					FilePath = cur;
				}
				foreach (Option option in Enum.GetValues(typeof(Option)))
				{
					if (!options.ContainsKey(option))
					{
						options[option] = false;
					}
				}
			}
			public enum Option
			{
				Format,
				SaveToFile,
			}
			public enum Command
			{
				Decode,
				Encode,
			}
			public bool HasOption(Option option)
			{
				return options[option];
			}
			private static Option? OptionOf(string arg)
			{
				Option option;
				if (optionMap.TryGetValue(arg, out option))
				{
					return option;
				}
				return null;
			}
			private static Command? CommandOf(string arg)
			{
				Command command;
				if (commandMap.TryGetValue(arg, out command))
				{
					return command;
				}
				return null;
			}
		}
	}
}
