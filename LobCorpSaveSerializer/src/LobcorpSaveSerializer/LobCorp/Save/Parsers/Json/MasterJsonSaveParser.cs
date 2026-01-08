using System;
using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class MasterJsonSaveParser : JsonSaveParserBase
	{
		public MasterJsonSaveParser() : base(SaveType.Master)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			throw new NotImplementedException();
		}
	}
}
