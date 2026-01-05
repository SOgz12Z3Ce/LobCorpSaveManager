using System;
using System.Collections.Generic;
using System.IO;
using LobCorp.Exceptions;
using LobCorp.Save;

namespace LobCorp
{
	internal class Program
	{
		static int Main(string[] args)
		{
			if (args.Length == 0)
			{
				Console.WriteLine("Usage: lobcss [options] file");
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
			using (FileStream fileStream = File.Open(parameter.FilePath, FileMode.Open))
			{
				LobCorpSaveSegment lobCorpSave = LobCorpSaveSerializer.Deserialize(fileStream);
				if (parameter.HasOption(Parameter.Option.Format))
				{
					Console.WriteLine(lobCorpSave.ToJson(Newtonsoft.Json.Formatting.Indented));
				}
				else
				{
					Console.WriteLine(lobCorpSave.ToJson());
				}
			}
			return 0;
		}
		private class Parameter
		{
			private static readonly Dictionary<string, Option> stringOptionMap = new Dictionary<string, Option>()
			{
				{ "-f", Option.Format},
				{ "--format", Option.Format},
			};
			private readonly string _filePath;
			private readonly Dictionary<Option, bool> options = new Dictionary<Option, bool>();
			public Parameter(string[] args)
			{
				for (int i = 0; i < args.Length; i++)
				{
					if (args[i].StartsWith("-"))
					{
						Option? option = OptionOf(args[i]);
						if (option == null)
						{
							throw new BadArguments(string.Format("Unable to resolve option: {0}", args[i]));
						}
						if (options.ContainsKey(option.Value))
						{
							throw new BadArguments(string.Format("Repeating option: {0}", args[i]));
						}
						options[option.Value] = true;
						continue;
					}
					if (_filePath != null)
					{
						throw new BadArguments(string.Format("Too many arguments: {0}", args[i]));
					}
					_filePath = args[i];
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
			}
			public string FilePath
			{
				get
				{
					return _filePath;
				}
			}
			public bool HasOption(Option option)
			{
				return options[option];
			}
			private static Option? OptionOf(string arg)
			{
				Option option;
				if (stringOptionMap.TryGetValue(arg, out option))
				{
					return option;
				}
				return null;
			}
		}
	}
}
