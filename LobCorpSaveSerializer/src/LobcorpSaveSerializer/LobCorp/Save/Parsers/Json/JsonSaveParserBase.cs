using System.Collections.Generic;
using System.Linq;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public abstract class JsonSaveParserBase
	{
		private readonly SaveType type;
		public JsonSaveParserBase(SaveType type)
		{
			this.type = type;
		}
		public bool TryParse(JObject save, out Dictionary<string, object> data)
		{
			if (!CanParse(save))
			{
				data = null;
				return false;
			}
			data = Parse(save);
			return true;
		}
		protected bool CanParse(JObject save)
		{
			return SaveTypeInferer.Infer(save) == type;
		}
		protected abstract Dictionary<string, object> Parse(JObject save);
		protected void Copy<T>(Dictionary<string, object> dest, JObject src, string attr)
		{
			dest[attr] = src[attr].Value<T>();
		}
		protected void CopyList<T>(Dictionary<string, object> dest, JObject src, string attr)
		{
			dest[attr] = src[attr].Values<T>().ToList();
		}
	}
}
