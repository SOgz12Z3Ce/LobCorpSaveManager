using System;
using System.Collections.Generic;
using LobCorp.Exceptions;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save
{
	public class LobCorpSaveSegment
	{
		public readonly Dictionary<string, object> data;
		private static readonly Dictionary<string, SaveType> inferMap = new Dictionary<string, SaveType>()
		{
			{ "bgmVolume", SaveType.Settings },
			{ "sefirabossTutorialPlayed", SaveType.Etc },
			{ "playTime", SaveType.Master },
			{ "observe", SaveType.Global },
			{ "money", SaveType.Unlimited },
			{ "genDay", SaveType.WhiteNight },
		};
		private static readonly Dictionary<SaveType, IJsonParser> jsonParserMap = new Dictionary<SaveType, IJsonParser>()
		{
			{ SaveType.Settings, new SettingsJsonParser() },
		};
		private static readonly Dictionary<SaveType, string> defaultBinaryFileNameMap = new Dictionary<SaveType, string>()
		{
			{ SaveType.Settings, "Lobotomy170808state.dat" },
		};
		private SaveType? _type;
		private string _defaultBinaryFileName;
		public LobCorpSaveSegment(Dictionary<string, object> data)
		{
			this.data = data;
		}
		public LobCorpSaveSegment(JObject json)
		{
			foreach (var kvp in jsonParserMap)
			{
				if (kvp.Value.TryParse(json, out data))
				{
					_type = kvp.Key; 
					return;
				}
			}
			throw new BadSaveFile("Unable to parser json.");
		}
		public LobCorpSaveSegment(string json) : this(JToken.Parse(json) as JObject)
		{
		}
		public enum SaveType
		{
			Settings,
			Etc,
			Master,
			Global,
			Unlimited,
			WhiteNight,
		}
		private interface IJsonParser
		{
			bool TryParse(JObject save, out Dictionary<string, object> data);
		}
		public SaveType Type
		{
			get
			{
				if (_type == null)
				{
					_type = InferType(data);
				}
				return _type.Value;
			}
		}
		public string DefaultBinaryFileName
		{
			get
			{
				if (_defaultBinaryFileName == null)
				{
					_defaultBinaryFileName = DefaultBinaryFileNameOf(Type);
				}
				return _defaultBinaryFileName;
			}
		}
		public static string DefaultBinaryFileNameOf(SaveType type)
		{
			return defaultBinaryFileNameMap[type];
		}
		public string ToJson()
		{
			return JsonConvert.SerializeObject(data);
		}
		public string ToJson(Formatting formatting)
		{
			return JsonConvert.SerializeObject(data, formatting);
		}
		private static SaveType InferType(Dictionary<string, object> save)
		{
			return InferType(save.ContainsKey);
		}
		private static SaveType InferType(JObject save)
		{
			return InferType(save.ContainsKey);
		}
		private static SaveType InferType(Func<string, bool> ContainsKey)
		{
			foreach (var kvp in inferMap)
			{
				if (ContainsKey(kvp.Key))
				{
					return kvp.Value;
				}
			}
			throw new BadSaveFile("Unable to infer save type.");
		}
		private class SettingsJsonParser : IJsonParser
		{
			public bool TryParse(JObject save, out Dictionary<string, object> data)
			{
				if (InferType(save) != SaveType.Settings)
				{
					data = null;
					return false;
				}
				Dictionary<string, object> result = new Dictionary<string, object>();

				result["bgmVolume"] = save["bgmVolume"].Value<float>();
				result["masterVolume"] = save["masterVolume"].Value<float>();
				result["tooltip"] = save["tooltip"].Value<bool>();
				result["language"] = save["language"].Value<string>();
				result["logIndex"] = save["logIndex"].Value<int>();

				data = result;
				return true;
			}
		}
	}
}
