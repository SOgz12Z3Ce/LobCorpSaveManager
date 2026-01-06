using System;
using System.CodeDom;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting;
using LobCorp.Exceptions;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;

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
		private static readonly Dictionary<SaveType, JsonParserBase> jsonParserMap = new Dictionary<SaveType, JsonParserBase>()
		{
			{ SaveType.Settings, new SettingsJsonParser() },
			{ SaveType.Etc, new EtcJsonParser() },
			{ SaveType.Master, new MasterJsonParser() },
			{ SaveType.Global, new GlobalJsonParser() },
			{ SaveType.WhiteNight, new WhiteNightJsonParser() },
		};
		private static readonly Dictionary<SaveType, string> defaultBinaryFileNameMap = new Dictionary<SaveType, string>()
		{
			{ SaveType.Settings, "Lobotomy170808state.dat" },
			{ SaveType.Etc, "etc170808.dat" },
			{ SaveType.Master, "saveData170808.dat" },
			{ SaveType.Global, "saveGlobal170808.dat" },
			{ SaveType.WhiteNight, "100014.dat" },
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
		private abstract class JsonParserBase
		{
			private readonly SaveType type;
			public JsonParserBase(SaveType type)
			{
				this.type = type;
			}
			public abstract bool TryParse(JObject save, out Dictionary<string, object> data);
			protected bool CanParse(JObject save)
			{
				return InferType(save) == type;
			}
			protected void Copy<T>(Dictionary<string, object> dest, JObject src, string attr)
			{
				dest[attr] = src[attr].Value<T>();
			}
			protected void CopyList<T>(Dictionary<string, object> dest, JObject src, string attr)
			{
				dest[attr] = src[attr].Values<T>().ToList();
			}
		}
		private class SettingsJsonParser : JsonParserBase
		{
			public SettingsJsonParser() : base(SaveType.Settings)
			{
			}
			public override bool TryParse(JObject save, out Dictionary<string, object> data)
			{
				if (!CanParse(save))
				{
					data = null;
					return false;
				}
				data = new Dictionary<string, object>();

				Copy<float>(data, save, "bgmVolume");
				Copy<float>(data, save, "bgmVolume");
				Copy<float>(data, save, "masterVolume");
				Copy<bool>(data, save, "tooltip");
				Copy<string>(data, save, "language");
				Copy<int>(data, save, "logIndex");

				return true;
			}
		}
		private class EtcJsonParser : JsonParserBase
		{
			public EtcJsonParser() : base(SaveType.Etc)
			{
			}
			public override bool TryParse(JObject save, out Dictionary<string, object> data)
			{
				if (!CanParse(save))
				{
					data = null;
					return false;
				}
				data = new Dictionary<string, object>();

				Copy<string>(data, save, "sefirabossTutorialPlayed");
				Copy<bool>(data, save, "e0");
				Copy<bool>(data, save, "e1");
				Copy<bool>(data, save, "e2");
				Copy<bool>(data, save, "e3");
				Copy<bool>(data, save, "e4");
				CopyList<long>(data, save, "waitingCreature");

				return true;
			}
		}
		private class MasterJsonParser : JsonParserBase
		{
			public MasterJsonParser() : base(SaveType.Master)
			{
			}
			public override bool TryParse(JObject save, out Dictionary<string, object> data)
			{
				throw new NotImplementedException();
			}
		}
		private class GlobalJsonParser : JsonParserBase
		{
			public GlobalJsonParser() : base(SaveType.Global)
			{
			}
			public override bool TryParse(JObject save, out Dictionary<string, object> data)
			{
				throw new NotImplementedException();
			}
		}
		private class UnlimitedJsonParser : JsonParserBase
		{
			public UnlimitedJsonParser() : base(SaveType.Unlimited)
			{
			}
			public override bool TryParse(JObject save, out Dictionary<string, object> data)
			{
				throw new NotImplementedException();
			}
		}
		private class WhiteNightJsonParser : JsonParserBase
		{
			public WhiteNightJsonParser() : base(SaveType.WhiteNight)
			{
			}
			public override bool TryParse(JObject save, out Dictionary<string, object> data)
			{
				throw new NotImplementedException();
			}
		}
	}
}
