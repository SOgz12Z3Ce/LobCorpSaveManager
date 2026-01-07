using System.Collections.Generic;
using LobCorp.Exceptions;
using LobCorp.Save.Parsers.Json;
using LobCorp.Save.Serializers;
using LobCorp.Save.Type;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save
{
	public class SaveFile
	{
		public readonly Dictionary<string, object> data;
		private SaveType? _type;
		private string _defaultBinaryFileName;
		public SaveFile(Dictionary<string, object> data)
		{
			this.data = data;
		}
		public SaveFile(JObject json)
		{
			foreach (var kvp in JsonSaveParserRegistry.ParsersMap)
			{
				if (kvp.Value.TryParse(json, out data))
				{
					_type = kvp.Key;
					return;
				}
			}
			throw new BadSaveFile("Unable to parser json.");
		}
		public SaveFile(string json) : this(JObject.Parse(json))
		{
		}
		public SaveType Type
		{
			get
			{
				if (_type == null)
				{
					_type = SaveTypeInferer.Infer(data);
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
					_defaultBinaryFileName = BinarySaveSerializer.DefaultFileNameOf(Type);
				}
				return _defaultBinaryFileName;
			}
		}
		public string ToJson()
		{
			return JsonConvert.SerializeObject(data);
		}
		public string ToJson(Formatting formatting)
		{
			return JsonConvert.SerializeObject(data, formatting);
		}
	}
}
